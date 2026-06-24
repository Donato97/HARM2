import { For, Show } from "solid-js";
import { EditorFile, EditorFolder, EFS } from ".";
import FolderMenu from "./FolderMenu";
import { EFSClient } from "./client";
import { editor } from "../index";

type FolderProps = {
    path: string;
    folder: EditorFolder;
};

type FileProps = {
    path: string;
    file: EditorFile;
};

export default function EditorFileSystem() {
    return (
        <div class="drawer-side overflow-y-auto shrink-0 grow-0">
            <label
                for="file-system-drawer"
                aria-label="close sidebar"
                class="drawer-overlay w-full"
            ></label>

            <div class="w-65 bg-base-300 border-r border-base-content/20 h-full">
                <div class="flex items-center justify-end p-2 border-b border-base-content/20">
                    <button
                        class="btn btn-xs btn-ghost"
                        onclick={() => EFS.addRootFolder()}
                    >
                        <span class="icon-[material-symbols--create-new-folder-outline] size-4" />
                    </button>
                    <button
                        class="btn btn-xs btn-ghost"
                        onclick={() => EFS.openAllFolders(EFS.fileSystem)}
                    >
                        <span class="icon-[material-symbols--expand] size-4" />
                    </button>
                    <button
                        class="btn btn-xs btn-ghost"
                        onclick={() => EFS.closeAllFolders(EFS.fileSystem)}
                    >
                        <span class="icon-[material-symbols--compress] size-4" />
                    </button>
                </div>

                <ul class="menu menu-sm w-full">
                    <For each={Object.entries(EFS.fileSystem)}>
                        {([key, folder]) => (
                            <li>
                                <Folder folder={folder} path={key} />
                            </li>
                        )}
                    </For>
                </ul>
            </div>
        </div>
    );
}

function Folder(props: FolderProps) {
    function onblur() {
        if (props.folder.name.trim() === "") {
            EFS.removeFolder(props.path);
            return;
        }
        EFS.toggleFolderEditMode(props.path);
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
                <button class="peer" onClick={() => EFS.toggleOpen(props.path)}>
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

function File(props: FileProps) {
    function onblur() {
        if (props.file.name.trim() === "") {
            EFS.removeFile(props.path);
            return;
        }
        EFS.toggleFileEditMode(props.path);
    }

    function onkeydown(e: KeyboardEvent) {
        if (e.key !== "Enter") return;

        e.preventDefault();
        document.getElementById("rename-file")?.blur();
    }

    return (
        <>
            <Show when={!props.file.editMode}>
                <button
                    onclick={async () => {
                        EFS.setActiveNote({
                            path: props.path,
                            file: props.file,
                        });
                        const content = await EFSClient.fetchFile(
                            props.file.id,
                        );
                        const state = editor.parseEditorState(content);
                        editor.setEditorState(state);
                    }}
                    class="peer"
                >
                    <span class="icon-[material-symbols--notes] size-4 shrink-0 grow-0" />
                    <span class="truncate">{props.file.name}</span>
                </button>

                {/* <FolderMenu folder={props.folder} path={props.path} /> */}
            </Show>

            <Show when={props.file.editMode}>
                <div class="flex items-center">
                    <span class="icon-[material-symbols--notes] size-4 shrink-0 grow-0" />
                    <input
                        id="rename-file"
                        type="text"
                        value={props.file.name}
                        oninput={(e) => (props.file.name = e.target.value)}
                        onkeydown={onkeydown}
                        onblur={onblur}
                        class="border-b border-primary cursor-text w-full"
                    />
                </div>
            </Show>
        </>
    );
}
