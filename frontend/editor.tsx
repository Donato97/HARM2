import { render } from "solid-js/web";
import Shell from "./editor/Shell";

let dispose: (() => void) | null = null;
let mounted: HTMLElement | null = null;

export function mountEditor(el: HTMLElement) {
    if (mounted === el) return;

    unmountEditor();
    mounted = el;
    dispose = render(() => <Shell />, el);
}

export function unmountEditor() {
    dispose?.();
    dispose = null;
    mounted = null;
}
