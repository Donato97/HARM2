import {
    $createParagraphNode,
    $getNodeByKey,
    $getSelection,
    $isNodeSelection,
    COMMAND_PRIORITY_LOW,
    DecoratorNode,
    defineExtension,
    EditorConfig,
    KEY_BACKSPACE_COMMAND,
    KEY_DELETE_COMMAND,
    LexicalEditor,
    mergeRegister,
    NodeKey,
    SerializedLexicalNode,
    Spread
} from "lexical";
import { debounce } from "ts-debounce";

type SerializedMermaidNode = Spread<
    { code: string },
    SerializedLexicalNode
>;

export class MermaidNode extends DecoratorNode<HTMLElement> {
    code: string;

    constructor(code: string, key?: NodeKey) {
        super(key);
        this.code = code;
    }

    static getType() {
        return "mermaid";
    }

    static clone(node: MermaidNode): MermaidNode {
        return new MermaidNode(node.code, node.__key);
    }

    static importJSON(json: SerializedMermaidNode): MermaidNode {
        return new MermaidNode(json.code || "");
    }

    exportJSON(): SerializedMermaidNode {
        return { ...super.exportJSON(), type: "mermaid", version: 1, code: this.code };
    }

    createDOM(_: EditorConfig, editor: LexicalEditor): HTMLElement {
        const container = document.createElement("div");
        container.classList.add(
            "my-6",
            "rounded-box",
            "bg-base-200",
            "p-2",
            "flex",
            "flex-col",
            "gap-2",
            "items-end"
        );
        const textarea = document.createElement("textarea");
        textarea.value = this.code;
        textarea.classList.add(
            "w-full",
            "resize-none",
            "font-mono!",
            "text-xs",
            "overflow-y-hidden"
        );
        textarea.addEventListener("keydown", (e) => {
            e.stopPropagation();
            if (e.key === "Enter" && e.shiftKey) {
                e.preventDefault();
                textarea.blur();
                editor.update(() => {
                    const self = $getNodeByKey(this.getKey());
                    if (!self) return;

                    const newP = $createParagraphNode();
                    self.insertAfter(newP);
                    newP.selectStart();
                });
            }
        });

        const preview = document.createElement("div");
        preview.classList.add("w-full", "h-fit");

        const menuItems = [
            {
                label: "Zoom",
                active: false,
                action: () => {
                    const dialog = document.createElement("dialog");
                    dialog.classList.add("modal");
                    dialog.addEventListener("close", () => dialog.remove());
                    const modalBox = document.createElement("div");
                    modalBox.classList.add("modal-box", "w-11/12", "max-w-5xl");
                    const closeButton = document.createElement("button");
                    closeButton.classList.add(
                        "btn",
                        "btn-sm",
                        "btn-circle",
                        "absolute",
                        "right-2",
                        "top-2"
                    );
                    closeButton.textContent = "✕";
                    closeButton.addEventListener("click", () => dialog.close());

                    modalBox.appendChild(closeButton);
                    modalBox.appendChild(preview.cloneNode(true));

                    dialog.appendChild(modalBox);
                    container.appendChild(dialog);
                    dialog.showModal();
                }
            },
            {
                label: "Code",
                active: false,
                action: () => {
                    textarea.remove();
                    preview.remove();
                    container.appendChild(textarea);
                    autoGrow();
                }
            },
            {
                label: "Preview",
                active: true,
                action: () => {
                    textarea.remove();
                    preview.remove();
                    container.appendChild(preview);
                },
            },
            {
                label: "Both",
                active: false,
                action: () => {
                    textarea.remove();
                    preview.remove();
                    container.appendChild(textarea);
                    container.appendChild(preview);
                    autoGrow();
                }
            },
        ];

        const menu = document.createElement("ul");
        menu.classList.add(
            "menu",
            "menu-horizontal",
            "menu-xs",
            "m-0!",
            "p-1",
            "bg-base-100",
            "rounded-box"
        );
        menuItems.forEach((item) => {
            const li = document.createElement("li");
            const button = document.createElement("button");
            button.textContent = item.label;
            button.addEventListener("click", item.action);
            li.classList.add("m-0!", "p-0!");
            li.appendChild(button);
            menu.appendChild(li);
        });


        container.appendChild(menu);
        container.appendChild(preview);

        const autoGrow = () => {
            textarea.style.height = "auto";
            textarea.style.height = textarea.scrollHeight + "px";
        };

        textarea.addEventListener("input", () => {
            debounceRenderMermaid(preview, textarea.value);
            this.code = textarea.value;
            autoGrow();
        });

        requestAnimationFrame(() => {
            autoGrow();
            renderMermaid(preview, textarea.value);
        });

        return container;
    }

    updateDOM() {
        return false;
    }

    isInline() {
        return false;
    }

    isKeyboardSelectable() {
        return false;
    }
}

export const MermaidExtension = defineExtension({
    name: "mermaid",
    nodes: [MermaidNode],
    register(editor) {
        const blockDeletion = () => {
            const selection = $getSelection();
            // se è selezionato un MermaidNode come nodo, blocca la cancellazione
            if ($isNodeSelection(selection)) {
                const nodes = selection.getNodes();
                if (nodes.some($isMermaidNode)) {
                    return true; // true = "gestito", blocca il comportamento di default
                }
            }
            return false;
        };

        return mergeRegister(
            editor.registerCommand(KEY_BACKSPACE_COMMAND, blockDeletion, COMMAND_PRIORITY_LOW),
            editor.registerCommand(KEY_DELETE_COMMAND, blockDeletion, COMMAND_PRIORITY_LOW),
        );
    },
})

export function $createMermaidNode(code?: string): MermaidNode {
    return new MermaidNode(code ?? "");
}

export function $isMermaidNode(node: any): node is MermaidNode {
    return node instanceof MermaidNode;
}

let mermaidIstance: typeof import("mermaid") | undefined;

async function getMermaid() {
    if (!mermaidIstance) {
        mermaidIstance = await import("mermaid");
        mermaidIstance.default.initialize({ startOnLoad: false, theme: "dark", securityLevel: "strict" });
    }
    return mermaidIstance;
}

async function renderMermaid(previewEl: HTMLElement, code: string) {
    if (!previewEl.isConnected) return;

    const prevHeight = previewEl.offsetHeight;
    previewEl.style.height = prevHeight + "px";

    try {
        const id = "mermaid-" + crypto.randomUUID();
        const mermaid = (await getMermaid()).default;
        const { svg } = await mermaid.render(id, code.trim(), previewEl);
        previewEl.innerHTML = svg;
    } catch (e) {
        previewEl.textContent = e instanceof Error ? e.message : String(e);
    } finally {
        previewEl.style.height = "auto";
    }
}

const debounceRenderMermaid = debounce(renderMermaid, 800);