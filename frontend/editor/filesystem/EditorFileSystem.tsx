import { Show } from "solid-js";
import { EditorFile, EditorFolder, EFS } from "./filesystem";
import FolderMenu from "./FolderMenu";

type FolderProps = {
  key: string;
  path: string;
  folder: EditorFolder;
};

type FileProps = {
  path: string;
  file: EditorFile;
};

export default function EditorFileSystem() {
  return (
    <div class="drawer-side transition-none bg-neutral/20 w-65 h-full overflow-y-auto shrink-0 grow-0 border-r border-stone-800">
      <ul class="menu menu-sm transition-none w-full">
        <li>
          <Folder folder={EFS.fileSystem.root} path="root" key="root" />
        </li>
      </ul>
    </div>
  );
}

function Folder(props: FolderProps) {
  console.log(props.path);

  function onBlur() {
    if (props.folder.name.trim() === "") {
      EFS.removeFolder(props.path)
      return;
    }
    props.folder.editMode = false;
  }

  return (
    <>
      <Show when={!props.folder.editMode}>
        <button class="peer" onClick={() => EFS.toggleOpen(props.path)}>
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
          {Object.entries(props.folder.folders).map(([key, folder]) => (
            <li>
              <Folder folder={folder} path={`${props.path}/${key}`} key={key} />
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


