import { createSignal, Show } from "solid-js";
import { EditorFile, EditorFolder, useEditorFileSystem } from "./filesystem";

type FolderProps = {
  path: string;
  folder: EditorFolder;
};

type FileProps = {
  path: string;
  file: EditorFile;
};

export default function EditorFileSystem() {
  const EFS = useEditorFileSystem();

  return (
    <div class="drawer-side transition-none bg-neutral/20 w-65 h-full overflow-y-auto shrink-0 grow-0 border-r border-stone-800">
      <ul class="menu menu-sm transition-none w-full">
        <li>
          <Folder folder={EFS.fileSystem.root} path="/root" />
        </li>
      </ul>
    </div>
  );
}

function Folder(props: FolderProps) {
  console.log(props.path);

  function toggleOpen() {
    props.folder.open = !props.folder.open;
  }

  //! TODO: remove the "new folder" property if the folder name is empty
  function onBlur() {
    if (props.folder.name.trim() === "") {
      delete props.folder["new folder"];
      return;
    }
    props.folder.editMode = false;
  }

  return (
    <>
      <Show when={!props.folder.editMode}>
        <button class="peer" onClick={toggleOpen}>
          <span
            class="size-4"
            classList={{
              "icon-[material-symbols--folder-outline]": !props.folder.open,
              "icon-[material-symbols--folder-open]": props.folder.open,
            }}
          />
          {props.folder.name}
        </button>

        <FolderMenu folder={props.folder} path={props.path} />
      </Show>

      <Show when={props.folder.editMode}>
        <div class="flex items-center">
          <span
            class="size-4"
            classList={{
              "icon-[material-symbols--folder-outline]": !props.folder.open,
              "icon-[material-symbols--folder-open]": props.folder.open,
            }}
          />
          <input
            id="rename-folder"
            type="text"
            value={props.folder.name}
            oninput={(e) => (props.folder.name = e.target.value)}
            onblur={onBlur}
            class="border cursor-pointer"
          />
        </div>
      </Show>

      <Show when={props.folder.open}>
        <ul>
          {Object.values(props.folder.folders).map((folder) => (
            <li>
              <Folder folder={folder} path={`${props.path}/${folder.name}`} />
            </li>
          ))}
          {Object.values(props.folder.files).map((file) => (
            <li>
              <File file={file} path={`${props.path}/${file.name}`} />
            </li>
          ))}
        </ul>
      </Show>
    </>
  );
}

function File(props: FileProps) {
  console.log(props.path);
  return (
    <a>
      <span class="icon-[material-symbols--markdown-outline] size-4" />
      {props.file.name}
    </a>
  );
}

function FolderMenu(props: { folder: EditorFolder; path: string }) {
  const [open, setOpen] = createSignal(false);

  function toggleOpen() {
    setOpen((prev) => !prev);
  }

  function onToggle(e: ToggleEvent) {
    setOpen(e.newState === "open");
  }

  function newFolder() {
    const duplicate = props.folder.folders["new folder"];

    if (duplicate) {
      return;
    }

    props.folder.folders["new folder"] = {
      name: "",
      open: false,
      editMode: true,
      files: {},
      folders: {},
    };

    const el = document.getElementById("rename-folder");
    if (el) {
      el.focus();
      toggleOpen();
    }
  }

  function newFile() {
    const duplicate = props.folder.files["new folder"];

    if (duplicate) {
      return;
    }

    props.folder.files["new folder"] = {
      name: "New file",
      content: "",
    };
  }

  return (
    <div class="opacity-0 peer-hover:opacity-100 hover:opacity-100 absolute right-0 top-0 p-0">
      <button
        class="btn btn-xs btn-ghost"
        onclick={toggleOpen}
        popoverTarget={props.path}
        style={{ "anchor-name": `--${props.path}` }}
      >
        <span class="icon-[material-symbols--more-vert] size-4" />
      </button>

      <Show when={open()}>
        <ul
          class="dropdown m-0 menu rounded-box bg-base-300 shadow-sm"
          popover="auto"
          ontoggle={onToggle}
          id={props.path}
          style={{ "position-anchor": `--${props.path}` }}
        >
          <li>
            <button onClick={newFolder}>
              <span class="icon-[material-symbols--create-new-folder-outline] size-4" />
              New folder
            </button>
          </li>
          <li>
            <button onClick={newFile}>
              <span class="icon-[material-symbols--add-notes] size-4" />
              New file
            </button>
          </li>
        </ul>
      </Show>
    </div>
  );
}
