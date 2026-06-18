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
    <div class="bg-neutral/20 w-65 h-full overflow-y-auto shrink-0 grow-0 border-r border-stone-800">
      <ul class="menu menu-sm w-full">
        <li>
          <Folder folder={EFS.fileSystem.root} path="" />
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

  return (
    <>
      <button onClick={toggleOpen}>
        <span
          class="size-4"
          classList={{
            "icon-[material-symbols--folder-outline]": !props.folder.open,
            "icon-[material-symbols--folder-open]": props.folder.open,
          }}
        />
        {props.folder.name}
      </button>

      <FolderMenu path={props.path} />

      <Show when={props.folder.open}>
        <ul>
          {Object.values(props.folder.files).map((file) => (
            <li>
              <File file={file} path={`${props.path}/${file.name}`} />
            </li>
          ))}
          {Object.values(props.folder.folders).map((folder) => (
            <li>
              <Folder folder={folder} path={`${props.path}/${folder.name}`} />
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

function FolderMenu(props: { path: string }) {
  const [open, setOpen] = createSignal(false);

  function toggleOpen() {
    setOpen((prev) => !prev);
  }

  function onToggle(e: ToggleEvent) {
    setOpen(e.newState === "open");
  }

  return (
    <div class="absolute right-0 top-0 p-0">
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
            <a>Item 1</a>
          </li>
          <li>
            <a>Item 2</a>
          </li>
        </ul>
      </Show>
    </div>
  );
}
