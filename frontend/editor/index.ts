import { buildEditorFromExtensions } from "@lexical/extension";
import { ListItemNode, ListNode } from "@lexical/list";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { createEditor, CreateEditorArgs } from "lexical";
import { ImageExtension } from "./plugins/image";

/* const config: CreateEditorArgs = {
    namespace: "MyEditor",
    nodes: [HeadingNode, QuoteNode, ListNode, ListItemNode],
    onError: console.error,
};

export const editor = createEditor(config); */

export const editor = buildEditorFromExtensions({
    name: "[root]",                                  // <-- nuovo: nome dell'extension root
    namespace: "MyEditor",
    nodes: [HeadingNode, QuoteNode, ListNode, ListItemNode],
    onError: console.error,
    dependencies: [ImageExtension],                  // <-- nuovo: qui entrano le tue feature
});
