import {
    $createParagraphNode,
    $getRoot,
    EditorState,
    UpdateListenerPayload,
} from "lexical";
import { registerCheckList, registerList } from "@lexical/list";
import { registerRichText } from "@lexical/rich-text";
import { onCleanup, onMount } from "solid-js";
import SlashMenu from "./slash-menu/SlashMenu";
import Title from "./Title";
import { EFS } from "./filesystem";
import { debounce } from "@solid-primitives/scheduled";
import { editor } from "./index";

export default function Editor() {
    let editorRef!: HTMLDivElement;

    const save = debounce((e: UpdateListenerPayload) => {
        const id = EFS.activeNote()?.file.id;
        if (!id) return;

        if (e.mutatedNodes === null) return;

        EFS.note.client.update(id, {
            content: JSON.stringify(e.editorState),
        });
    }, 1000);

    onMount(() => {
        editor.setRootElement(editorRef);
        const cleanupRichText = registerRichText(editor);
        const cleanupList = registerList(editor);
        const cleanupCheckList = registerCheckList(editor, {
            disableTakeFocusOnClick: true,
        });

        editor.registerUpdateListener(save);

        editor.update(() => {
            const root = $getRoot();
            if (root.getFirstChild() === null) {
                root.append($createParagraphNode());
            }
        });

        onCleanup(() => {
            cleanupRichText();
            cleanupList();
            cleanupCheckList();
        });
    });

    return (
        <>
            <div class="w-[98%] sm:w-[90%] lg:w-[70%] mx-auto xl:max-w-2xl">
                <Title onEnter={() => editor.focus()} />

                <div
                    id="editor"
                    class="flex-1 min-h-0 p-4 mt-4 prose max-w-full"
                    contentEditable
                    spellcheck="false"
                    ref={editorRef}
                ></div>
            </div>
            <SlashMenu editor={editor} />
        </>
    );
}
