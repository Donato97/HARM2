import { createEditor, CreateEditorArgs } from "lexical";
import { HeadingNode, registerRichText } from "@lexical/rich-text";

import { onCleanup, onMount } from "solid-js";
import SlashMenu from "./slash-menu/SlashMenu";
import {
  ListItemNode,
  ListNode,
  registerCheckList,
  registerList,
} from "@lexical/list";

export default function Editor() {
  const config: CreateEditorArgs = {
    namespace: "MyEditor",
    nodes: [HeadingNode, ListNode, ListItemNode],
    onError: console.error,
  };

  let editorRef!: HTMLDivElement;
  let editor = createEditor(config);

  onMount(() => {
    editor.setRootElement(editorRef);
    const cleanupRichText = registerRichText(editor);
    const cleanupList = registerList(editor);
    const cleanupCheckList = registerCheckList(editor, {
      disableTakeFocusOnClick: true,
    });
    onCleanup(() => {
      cleanupRichText();
      cleanupList();
      cleanupCheckList();
    });
  });

  return (
    <main class="container h-screen flex flex-col items-center justify-center gap-6 mx-auto">
      <h1 class="text-3xl font-bold">HARM2 — SolidJS + Axum + Maud</h1>
      <div
        id="editor"
        class="h-1/2 w-full border border-base-100 bg-base-300 rounded-2xl p-4 overflow-y-auto"
        contentEditable
        ref={editorRef}
      ></div>

      <SlashMenu editor={editor} />
    </main>
  );
}
