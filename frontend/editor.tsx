import { render } from "solid-js/web";
import Shell from "./editor/Shell";

let dispose: (() => void) | null = null;

export function mountEditor(el: HTMLElement) {
    if (dispose) return;
    dispose = render(() => <Shell />, el);
}

export function unmountEditor() {
    dispose?.();
    dispose = null;
}
