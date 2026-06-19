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
    <main class="drawer md:drawer-open h-full flex">
      <input id="my-drawer-3" type="checkbox" class="drawer-toggle" />

      <EditorFileSystem />

      <div class="drawer-content h-full w-[98%] sm:w-[80%] md:w-1/2 mx-auto flex flex-col min-h-0 py-20 overflow-y-auto">
        <Title onEnter={() => editor.focus()} />

        <div
          id="editor"
          class="flex-1 min-h-0 w-full rounded-2xl p-4 mt-4"
          contentEditable
          ref={editorRef}
        ></div>

        <SlashMenu editor={editor} />
      </div>
    </main>
  );
}
