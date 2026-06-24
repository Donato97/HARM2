import { ListItemNode, ListNode } from "@lexical/list";
import { HeadingNode } from "@lexical/rich-text";
import { createEditor, CreateEditorArgs } from "lexical";

const config: CreateEditorArgs = {
    namespace: "MyEditor",
    nodes: [HeadingNode, ListNode, ListItemNode],
    onError: console.error,
};

export const editor = createEditor(config);
