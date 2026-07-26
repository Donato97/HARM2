import { onMount } from "solid-js";
import { editor } from "./index";

export default function Editor() {
    let editorRef!: HTMLDivElement;

    onMount(() => {
        editor.mount(editorRef);
    });

    return (
        <div class="w-[98%] sm:w-[90%] lg:w-[70%] mx-auto xl:max-w-2xl h-full">
            <div
                id="editor"
                class="flex-1 min-h-0 h-full p-4 prose max-w-full"
                spellcheck="false"
                ref={editorRef}
            ></div>
        </div>
    );
}
