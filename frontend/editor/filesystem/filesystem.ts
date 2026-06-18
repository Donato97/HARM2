import { createMutable, createStore } from "solid-js/store";

export type EditorFile = {
  name: string;
  content: string;
};

export type EditorFolder = {
  name: string;
  open: boolean;
  files: Record<string, EditorFile>;
  folders: Record<string, EditorFolder>;
};

export type EditorFileSystem = {
  root: EditorFolder;
};

export function useEditorFileSystem() {
  const fileSystem = createMutable<EditorFileSystem>({
    root: {
      name: "root",
      open: false,
      files: {
        pippo: { name: "pippo", content: "" },
      },
      folders: {
        test: {
          name: "test",
          open: false,
          files: {
            pippo: { name: "pippo", content: "" },
          },
          folders: {
            subfolder: {
              name: "subfolder",
              open: false,
              files: { pippo: { name: "pippo", content: "" } },
              folders: {},
            },
          },
        },
      },
    },
  });

  /* function addFile(path: string, file: EditorFile) {
    setStore("root", "files", path, file);
  }

  function addFolder(path: string, folder: EditorFolder) {
    setStore("root", "folders", path, folder);
  }

  function removeFile(path: string) {
    setStore("root", "files", path, undefined!);
  }

  function removeFolder(path: string) {
    setStore("root", "folders", path, undefined!);
  } */

  return { fileSystem };
}
