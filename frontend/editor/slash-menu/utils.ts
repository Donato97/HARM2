import { $createCodeNode } from "@lexical/code-core";
import { $createListNode } from "@lexical/list";
import { $createHeadingNode } from "@lexical/rich-text";
import { $getSelection, $isRangeSelection, $isTextNode } from "lexical";
import { $createMermaidNode } from "../plugins/mermaid";

export const slashMenuItems = [
  {
    icon: "icon-[material-symbols--format-h1]",
    name: "Heading 1",
    node: () => $createHeadingNode("h1"),
  },
  {
    icon: "icon-[material-symbols--format-h2]",
    name: "Heading 2",
    node: () => $createHeadingNode("h2"),
  },
  {
    icon: "icon-[material-symbols--format-h3]",
    name: "Heading 3",
    node: () => $createHeadingNode("h3"),
  },
  {
    icon: "icon-[material-symbols--format-h4]",
    name: "Heading 4",
    node: () => $createHeadingNode("h4"),
  },
  {
    icon: "icon-[material-symbols--format-list-numbered]",
    name: "Numbered list",
    node: () => $createListNode("number"),
  },
  {
    icon: "icon-[material-symbols--format-list-bulleted]",
    name: "Bullet list",
    node: () => $createListNode("bullet"),
  },
  {
    icon: "icon-[material-symbols--checklist]",
    name: "Check list",
    node: () => $createListNode("check"),
  },
  {
    icon: "icon-[material-symbols--code]",
    name: "Code",
    node: () => $createCodeNode(),
  },
  {
    icon: "icon-[material-symbols--mermaid]",
    name: "Mermaid",
    node: () => $createMermaidNode(),
  },
];

export function validatedSlashMenuQuery(): string | undefined {
  const selection = $getSelection();

  if (!$isRangeSelection(selection) || !selection.isCollapsed()) return;

  const node = selection.anchor.getNode();
  if (!$isTextNode(node)) return;

  const text = node.getTextContent().trim().toLocaleLowerCase();
  if (!text.startsWith("/")) return;

  return text.slice(1);
}
