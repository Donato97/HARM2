import { createMutable, createStore } from "solid-js/store";

export type EditorFile = {
  name: string;
  content: string;
};

export type EditorFolder = {
  id: string;
  name: string;
  open: boolean;
  editMode: boolean;
  files: Record<string, EditorFile>;
  folders: Record<string, EditorFolder>;
};

export type EditorFileSystem = Record<string, EditorFolder>;

function useEditorFileSystem() {
  const fileSystem = createMutable<EditorFileSystem>({
    root: {
      id: crypto.randomUUID(),
      name: "root",
      open: false,
      editMode: false,
      files: {
        pippo: { name: "pippo", content: "" },
      },
      folders: {
        test: {
          id: crypto.randomUUID(),
          name: "test",
          open: false,
          editMode: false,
          files: {
            pippo: { name: "pippo", content: "" },
          },
          folders: {
            subfolder: {
              id: crypto.randomUUID(),
              name: "subfolder",
              open: false,
              editMode: false,
              files: { pippo: { name: "pippo", content: "" } },
              folders: {},
            },
          },
        },
      },
    },
  });

  function toggleOpen(path: string) {
    const node = _getFolderNode(path);
    if (!node) return;
    node.open = !node.open;
  }

  function addFolder(path: string) {
    const node = _getFolderNode(path);
    if (!node) return;

    const key = crypto.randomUUID();

    node.open = true;
    node.folders[key] = {
      id: key,
      name: "",
      open: false,
      editMode: true,
      files: {},
      folders: {},
    };
  }

  function removeFolder(path: string) {
    const parts = path.split("/");
    const key = parts.at(-1);
    if (!key) return;

    const parentPath = parts.slice(0, -1).join("/")
    const node = _getFolderNode(parentPath);
    if (!node) return;

    delete node.folders[key];
  }

  function _getFolderNode(path: string): EditorFolder | undefined {
    const pathParts = path.split("/").slice(1);

    let node: EditorFolder | undefined = fileSystem.root;
    for (const part of pathParts) {
      node = node?.folders[part];
    }
    return node;
  }

  return { fileSystem, addFolder, removeFolder, toggleOpen };
}

export const EFS = useEditorFileSystem();
