import Alpine from "alpinejs";
import { editor } from "./index";

const editorStore = {
    activeId: null as string | null,

    set(id: string, content: string | null) {
        this.activeId = id;
        editor.commands.setContent(content ? JSON.parse(content) : '');
    },
    unset() {
        this.activeId = null;
        editor.commands.clearContent();
    },
}

Alpine.store('editor', editorStore);

export function store() {
    return Alpine.store('editor') as any;
}