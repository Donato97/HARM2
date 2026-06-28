import { For, Show } from "solid-js";
import { EditorFolder, EFS } from ".";
import FolderMenu from "./FolderMenu";
import File from "./File";

type FolderProps = {
    path: string;
    folder: EditorFolder;
};

export default function Folder(props: FolderProps) {
    function onblur() {
        if (props.folder.name.trim() === "") {
            EFS.folder.remove(props.path);
            return;
        }
        EFS.folder.toggleEditMode(props.path);
        EFS.folder.client.createOrUpdate({
            id: props.folder.id,
            name: props.folder.name,
            type_: "folder",
            parent_id: props.path.split("/").at(-2),
        });
    }

    function onkeydown(e: KeyboardEvent) {
        if (e.key !== "Enter") return;

        e.preventDefault();
        document.getElementById("rename-folder")?.blur();
    }

    function Icon() {
        return (
            <span
                class="size-4 shrink-0 grow-0"
                classList={{
                    "icon-[material-symbols--folder-outline]":
                        !props.folder.open,
                    "icon-[material-symbols--folder-open]": props.folder.open,
                }}
            />
        );
    }

    return (
        <>
            <Show when={!props.folder.editMode}>
                <button
                    class="peer"
                    onClick={() => EFS.folder.toggleOpen(props.path)}
                >
                    <Icon />
                    <span class="truncate">{props.folder.name}</span>
                </button>

                <FolderMenu folder={props.folder} path={props.path} />
            </Show>

            <Show when={props.folder.editMode}>
                <div class="flex items-center">
                    <Icon />
                    <input
                        id="rename-folder"
                        type="text"
                        value={props.folder.name}
                        oninput={(e) => (props.folder.name = e.target.value)}
                        onkeydown={onkeydown}
                        onblur={onblur}
                        class="border-b border-primary cursor-text w-full"
                    />
                </div>
            </Show>

            <Show when={props.folder.open}>
                <ul>
                    <For each={Object.entries(props.folder.folders)}>
                        {([key, folder]) => (
                            <li>
                                <Folder
                                    folder={folder}
                                    path={`${props.path}/${key}`}
                                />
                            </li>
                        )}
                    </For>
                    <For each={Object.entries(props.folder.files)}>
                        {([key, file]) => (
                            <li>
                                <File
                                    file={file}
                                    path={`${props.path}/${key}`}
                                />
                            </li>
                        )}
                    </For>
                </ul>
            </Show>
        </>
    );
}