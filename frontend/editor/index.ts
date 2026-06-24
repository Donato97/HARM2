import { ListItemNode, ListNode } from "@lexical/list";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { createEditor, CreateEditorArgs } from "lexical";

const config: CreateEditorArgs = {
    namespace: "MyEditor",
    nodes: [HeadingNode, QuoteNode, ListNode, ListItemNode],
    onError: console.error,
};

export const editor = createEditor(config);
