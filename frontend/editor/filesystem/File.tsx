import { EditorFile, EFS } from "./index";
import { editor } from "../index";
import { Show } from "solid-js";

type FileProps = {
    path: string;
    file: EditorFile;
};

export default function File(props: FileProps) {
    function onblur() {
        if (props.file.name.trim() === "") {
            EFS.note.remove(props.path);
            return;
        }
        EFS.note.toggleEditMode(props.path);
        EFS.folder.client.createOrUpdate({
            id: props.file.id,
            name: props.file.name,
            type_: "file",
            parent_id: props.path.split("/").at(-2),
        });
    }

    function onkeydown(e: KeyboardEvent) {
        if (e.key !== "Enter") return;

        e.preventDefault();
        document.getElementById("rename-file")?.blur();
    }

    async function onclick() {
        EFS.setActiveNote({
            path: props.path,
            file: props.file,
        });

        history.pushState({ htmx: true }, "", `/?note=${props.file.id}`);
    }

    return (
        <>
            <Show when={!props.file.editMode}>
                <button onclick={onclick} class="peer">
                    <span class="icon-[material-symbols--notes] size-4 shrink-0 grow-0" />
                    <span class="truncate">{props.file.name}</span>
                </button>
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
