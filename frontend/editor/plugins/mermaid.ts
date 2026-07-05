import { CodeNode, SerializedCodeNode } from "@lexical/code-core";
import { $getEditor, $getNodeByKey, defineExtension, EditorConfig, NodeKey } from "lexical";
import mermaid from "mermaid";

mermaid.initialize({ startOnLoad: false });

type DisplayMode = "code" | "preview";

export class MermaidNode extends CodeNode {
    __displayMode: DisplayMode = "code";

    constructor(displayMode: DisplayMode = "code", key?: NodeKey) {
        super("mermaid", key);
        this.__displayMode = displayMode;
    }

    static getType() {
        return "mermaid";
    }

    static clone(node: MermaidNode): MermaidNode {
        return new MermaidNode(node.__displayMode, node.__key);
    }

    static importJSON(): MermaidNode {
        return new MermaidNode();
    }

    exportJSON(): SerializedCodeNode {
        return super.exportJSON();
    }

    createDOM(config: EditorConfig): HTMLElement {
        const editor = $getEditor();
        const key = this.getKey();

        const wrapper = document.createElement("div");
        wrapper.classList.add("bg-base-200", "rounded-box");

        const previewEl = document.createElement("div");
        wrapper.appendChild(previewEl);

        const menu = document.createElement("ul");
        menu.classList.add("bg-base-100", "menu", "menu-xs", "menu-horizontal", "m-1!", "rounded-box", "p-1");
        const codeButton = document.createElement("button");
        codeButton.classList.add("btn", "btn-xs", "btn-ghost");
        codeButton.textContent = "Code";
        const previewButton = document.createElement("button");
        previewButton.classList.add("btn", "btn-xs", "btn-ghost");
        previewButton.textContent = "Preview";

        previewButton.addEventListener("click", () => {
            let code = "";
            editor.read(() => {
                const node = $getNodeByKey(key);
                code = node ? node.getTextContent() : "";
            });
            renderMermaid(previewEl, code);
        });


        menu.appendChild(codeButton);
        menu.appendChild(previewButton);
        wrapper.appendChild(menu);

        const code = super.createDOM(config);
        wrapper.appendChild(code);

        return wrapper;
    }

    getDOMSlot(element: HTMLElement) {
        return super.getDOMSlot(element.querySelector("code") as HTMLElement);
    }

    updateDOM(prevNode: this, dom: HTMLElement, config: EditorConfig): boolean {
        return super.updateDOM(prevNode, dom.querySelector("code") as HTMLElement, config);
    }
}

export const MermaidExtension = defineExtension({
    name: "mermaid",
    nodes: [MermaidNode],
    register(editor) { return () => { }; },
})

export function $createMermaidNode(): MermaidNode {
    return new MermaidNode();
}

export function $isMermaidNode(node: any): node is MermaidNode {
    return node instanceof MermaidNode;
}

async function renderMermaid(previewEl: HTMLElement, code: string) {
    try {
        const uuid = crypto.randomUUID();
        const { svg } = await mermaid.render(uuid, code.trim());
        previewEl.innerHTML = svg;
    } catch (e) {
        previewEl.textContent = e instanceof Error ? e.message : String(e);
    }
}