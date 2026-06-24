import { createSignal, Show } from "solid-js";
import { EditorFolder, EFS } from ".";

export default function FolderMenu(props: {
    folder: EditorFolder;
    path: string;
}) {
    const depth = props.path.split("/").length;
    const [open, setOpen] = createSignal(false);

    function toggleOpen() {
        setOpen((prev) => !prev);
    }

    function onToggle(e: ToggleEvent) {
        setOpen(e.newState === "open");
    }

    function newFolder() {
        EFS.addFolder(props.path);

        const el = document.getElementById("rename-folder");
        if (el) {
            el.focus();
            toggleOpen();
        }
    }

    function newFile() {
        EFS.addFile(props.path);

        const el = document.getElementById("rename-file");
        if (el) {
            el.focus();
            toggleOpen();
        }
    }

    function rename() {
        EFS.toggleFolderEditMode(props.path);

        const el = document.getElementById("rename-folder");
        if (el) {
            el.focus();
            toggleOpen();
        }
    }

    return (
        <div class="opacity-0 peer-hover:opacity-100 hover:opacity-100 absolute right-0 top-0 p-0">
            <button
                class="btn btn-xs btn-ghost"
                onclick={toggleOpen}
                popoverTarget={props.folder.id}
                style={{ "anchor-name": `--${props.folder.id}` }}
            >
                <span class="icon-[material-symbols--more-vert] size-4" />
            </button>

            <Show when={open()}>
                <ul
                    class="dropdown m-0 menu border border-base-content/20 rounded-box bg-base-100 shadow-sm"
                    popover="auto"
                    ontoggle={onToggle}
                    id={props.folder.id}
                    style={{ "position-anchor": `--${props.folder.id}` }}
                >
                    <Show when={depth < 5}>
                        <li>
                            <button onClick={newFolder}>
                                <span class="icon-[material-symbols--create-new-folder-outline] size-4" />
                                New folder
                            </button>
                        </li>
                    </Show>
                    <li>
                        <button onClick={newFile}>
                            <span class="icon-[material-symbols--add-notes] size-4" />
                            New file
                        </button>
                    </li>
                    <li>
                        <button onClick={rename}>
                            <span class="icon-[material-symbols--edit] size-4" />
                            Rename
                        </button>
                    </li>
                </ul>
            </Show>
        </div>
    );
}
