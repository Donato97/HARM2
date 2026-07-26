import {
    $createParagraphNode, $getRoot,
    $isDecoratorNode,
    UpdateListenerPayload
} from "lexical";
import { registerCheckList, registerList } from "@lexical/list";
import { registerRichText } from "@lexical/rich-text";
import { onCleanup, onMount } from "solid-js";
import SlashMenu from "./slash-menu/SlashMenu";
import { EFS } from "./filesystem";
import { debounce } from "@solid-primitives/scheduled";
import { editor } from "./index";

export default function Editor() {
    let editorRef!: HTMLDivElement;

    const save = debounce((e: UpdateListenerPayload) => {s
        const id = EFS.activeNote()?.file.id;
        if (!id) return;

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

        editor.getRootElement()?.addEventListener("click", () => {
            editor.update(() => {
                const rootNode = $getRoot();
                const last = rootNode.getLastChild();
                if (last && $isDecoratorNode(last)) {
                    const paragraph = $createParagraphNode();
                    rootNode.append(paragraph);
                    paragraph.selectStart();
                }
            });
        })

        onCleanup(() => {
            cleanupRichText();
            cleanupList();
            cleanupCheckList();
        });
    });


    return (
        <>
            <div class="w-[98%] sm:w-[90%] lg:w-[70%] mx-auto xl:max-w-2xl h-full">
                <div
                    id="editor"
                    class="flex-1 min-h-0 h-full p-4 prose max-w-full"
                    contentEditable
                    spellcheck="false"
                    ref={editorRef}
                ></div>
            </div>

            <SlashMenu editor={editor} />
        </>
    );
}
