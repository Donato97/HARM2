import {
  $createParagraphNode,
  $getRoot,
  createEditor,
  CreateEditorArgs,
} from "lexical";
import { HeadingNode, registerRichText } from "@lexical/rich-text";

import { onCleanup, onMount } from "solid-js";
import SlashMenu from "./slash-menu/SlashMenu";
import {
  ListItemNode,
  ListNode,
  registerCheckList,
  registerList,
} from "@lexical/list";
import Title from "./Title";
import EditorFileSystem from "./filesystem/EditorFileSystem";

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
    <main class="h-screen mx-auto overflow-y-auto flex">
      <EditorFileSystem />

      <div class="max-w-prose mx-auto py-20">
        <Title onEnter={() => editor.focus()} />

        <div
          id="editor"
          class="h-full w-full rounded-2xl p-4 mt-4"
          contentEditable
          ref={editorRef}
        ></div>

        <SlashMenu editor={editor} />
      </div>
    </main>
  );
}
