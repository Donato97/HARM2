import {
    $createParagraphNode,
    $getRoot,
    createEditor,
    CreateEditorArgs,
    EditorState,
} from "lexical";
import {
    ListItemNode,
    ListNode,
    registerCheckList,
    registerList,
} from "@lexical/list";
import { HeadingNode, registerRichText } from "@lexical/rich-text";
import { onCleanup, onMount } from "solid-js";
import SlashMenu from "./slash-menu/SlashMenu";
import Title from "./Title";
import { EFS } from "./filesystem";
import { EFSClient } from "./filesystem/client";
import { debounce } from "@solid-primitives/scheduled";
import { editor } from "./index";

export default function Editor() {
    let editorRef!: HTMLDivElement;

    const save = debounce((editorState: EditorState) => {
        const id = EFS.activeNote()?.file.id;
        if (!id) return;

        EFSClient.updateFile({
            id,
            content: JSON.stringify(editorState),
        });
    }, 1000);

    onMount(() => {
        editor.setRootElement(editorRef);
        const cleanupRichText = registerRichText(editor);
        const cleanupList = registerList(editor);
        const cleanupCheckList = registerCheckList(editor, {
            disableTakeFocusOnClick: true,
        });

        editor.registerUpdateListener(({ editorState }) => {
            save(editorState);
        });

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
            <Title onEnter={() => editor.focus()} />

            <div
                id="editor"
                class="flex-1 min-h-0 w-full rounded-2xl p-4 mt-4"
                contentEditable
                spellcheck="false"
                ref={editorRef}
            ></div>

            <SlashMenu editor={editor} />
        </>
    );
}
