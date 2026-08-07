import { createEffect, onMount } from "solid-js";
import { create_editor, editor } from "./index";
import { EFS } from "./filesystem";

export default function Editor() {
    let editorRef!: HTMLDivElement;

    createEffect(async () => {
        const note = EFS.activeNote();
        if (!note) return;

        const content = await EFS.note.client.find(note.file.id);
        editor.commands.setContent(
            content ? JSON.parse(content) : "<p>New file</p>",
        );
    });

    onMount(() => {
        create_editor(editorRef);
        EFS.openFromUrl();
    });

    return (
        <div class="py-20 sm:py-4 h-full">
            <div class="w-[95%] sm:w-[80%] lg:w-[70%] mx-auto xl:max-w-2xl h-full">
                <div
                    id="editor"
                    class="flex-1 min-h-0 h-full prose max-w-full"
                    spellcheck="false"
                    ref={editorRef}
                ></div>
            </div>
        </div>
    );
}
