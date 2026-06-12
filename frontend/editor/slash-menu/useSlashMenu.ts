import { $getSelection, $isRangeSelection, $isTextNode } from "lexical";
import { createMemo, createSignal } from "solid-js";
import { slashMenuItems, validatedSlashMenuQuery } from "./utils";
import { $setBlocksType } from "@lexical/selection";

export default function useSlashMenu() {
  const [isOpen, setIsOpen] = createSignal(false);
  const [selectedIndex, setSelectedIndex] = createSignal(0);
  const [slashMenuQuery, setSlashMenuQuery] = createSignal("");
  const pos = {
    x: 0,
    y: 0,
  };

  const filteredItems = createMemo(() =>
    slashMenuItems.filter((item) =>
      item.name.toLocaleLowerCase().includes(slashMenuQuery()),
    ),
  );

  function open() {
    const query = validatedSlashMenuQuery();
    if (query === undefined) {
      setIsOpen(false);
      return;
    }

    const rect = window.getSelection()?.getRangeAt(0).getBoundingClientRect();

    if (rect) {
      pos.x = rect.left;
      pos.y = rect.bottom;
      setIsOpen(true);
    }
    setSlashMenuQuery(query);
    setSelectedIndex(0);
  }

  function onEnterDown(e: KeyboardEvent) {
    if (!isOpen()) return false;
    e?.preventDefault();

    const selection = $getSelection();
    if (!$isRangeSelection(selection) || !selection.isCollapsed()) return false;

    const node = selection.anchor.getNode();
    if ($isTextNode(node)) node.setTextContent("");

    $setBlocksType(selection, filteredItems()[selectedIndex()].node);

    setIsOpen(false);
    return true;
  }

  function onArrowDown(e: KeyboardEvent) {
    const move = (delta: number) => {
      const len = filteredItems().length;
      if (len === 0) return false;
      setSelectedIndex((i) => (i + delta + len) % len);
      return true;
    };
    if (!isOpen()) return false;
    e.preventDefault();
    return move(e.key === "ArrowDown" ? 1 : -1);
  }

  return {
    open,
    isOpen,
    pos,
    selectedIndex,
    filteredItems,
    onEnterDown,
    onArrowDown,
  };
}
