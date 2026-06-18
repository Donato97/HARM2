import { render } from "solid-js/web";
import Editor from "./editor/Editor";

let dispose: (() => void) | null = null;

export function mountEditor(el: HTMLElement) {
  if (dispose) return;
  dispose = render(() => <Editor />, el);
}

export function unmountEditor() {
  dispose?.();
  dispose = null;
}
