import htmx from "htmx.org";

declare global {
    interface Window {
        htmx: typeof htmx;
    }
}

let editorModule: typeof import("./editor") | undefined;

async function syncEditor() {
    const root = document.querySelector<HTMLElement>("#root");

    if (!root && !editorModule) {
        return;
    } else {
        editorModule = await import("./editor");
    }

    const { mountEditor, unmountEditor } = editorModule;

    if (root) {
        mountEditor(root);
    } else {
        unmountEditor();
    }
}

syncEditor();
htmx.on(document.body, "htmx:after:settle", syncEditor);
