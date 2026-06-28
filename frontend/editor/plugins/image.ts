import { DRAG_DROP_PASTE } from "@lexical/rich-text";
import { $getNodeByKey, $insertNodes, COMMAND_PRIORITY_EDITOR, createCommand, DecoratorNode, defineExtension, EditorConfig, ElementFormatType, LexicalCommand, LexicalEditor, LexicalNode, mergeRegister, NodeKey, SerializedLexicalNode, Spread } from "lexical";

export type SerializedImageNode = Spread<
    { src: string; align: ElementFormatType },
    SerializedLexicalNode
>;

type InsertImagePayload = {
    src: string;
    align: ElementFormatType;
};

export class ImageNode extends DecoratorNode<HTMLElement> {
    __src: string;
    __align: ElementFormatType = "left";

    constructor(src: string, align: ElementFormatType, key?: NodeKey) {
        super(key);
        this.__src = src;
        this.__align = align;
    }

    static getType() {
        return "image";
    }

    static clone(node: ImageNode): ImageNode {
        return new ImageNode(node.__src, node.__align, node.__key);
    }

    static importJSON(json: SerializedImageNode): ImageNode {
        return new ImageNode(json.src, json.align);
    }

    exportJSON(): SerializedImageNode {
        return {
            ...super.exportJSON(),
            type: "image",
            version: 1,
            src: this.__src,
            align: this.__align,
        };
    }

    createDOM(_: EditorConfig, editor: LexicalEditor): HTMLElement {
        const p = document.createElement("p");
        p.classList.add("relative");
        p.style.textAlign = this.__align;

        const menu = document.createElement("ul");
        menu.classList.add("absolute", "top-5", "right-2", "bg-base-200", "menu", "menu-xs", "menu-horizontal", "shadow-lg", "p-1", "rounded-box");

        menu.addEventListener("mousedown", (e: MouseEvent) => {
            e.preventDefault();

            const btn = (e.target as HTMLElement).closest("button");
            if (!btn) return;

            const align = btn.dataset.align;
            if (!align) return;


            editor.update(() => {
                const node = $getNodeByKey(this.getKey());
                if ($isImageNode(node)) node.setAlign(align as ElementFormatType);
            });
        });

        const menuItem1 = document.createElement("li");
        menuItem1.classList.add("my-0!");
        const button1 = document.createElement("button");
        button1.dataset.align = "left";
        button1.textContent = "Left";
        menuItem1.appendChild(button1);
        const menuItem2 = document.createElement("li");
        menuItem2.classList.add("my-0!");
        const button2 = document.createElement("button");
        button2.dataset.align = "center";
        button2.textContent = "Center";
        menuItem2.appendChild(button2);
        const menuItem3 = document.createElement("li");
        menuItem3.classList.add("my-0!");
        const button3 = document.createElement("button");
        button3.dataset.align = "right";
        button3.textContent = "Right";
        menuItem3.appendChild(button3);

        menu.append(menuItem1, menuItem2, menuItem3);

        const img = document.createElement("img");
        img.src = this.__src;
        img.draggable = false;
        p.appendChild(img);

        p.addEventListener("mouseenter", () => p.appendChild(menu));
        p.addEventListener("mouseleave", () => p.removeChild(menu));
        return p;
    }

    updateDOM(prev: ImageNode): boolean {
        return prev.__align !== this.__align;
    }

    isInline(): boolean {
        return false;
    }

    setAlign(align: ElementFormatType): void {
        this.getWritable().__align = align;
    }
}

const INSERT_IMAGE_COMMAND: LexicalCommand<InsertImagePayload> =
    createCommand("INSERT_IMAGE_COMMAND");

export const ImageExtension = defineExtension({
    name: "image",
    nodes: [ImageNode],
    register(editor) {
        const command = editor.registerCommand(
            INSERT_IMAGE_COMMAND,
            (payload: InsertImagePayload) => {
                $insertNodes([
                    $createImageNode(payload.src, payload.align),
                ]);
                return true;
            },
            COMMAND_PRIORITY_EDITOR
        );
        const command2 = editor.registerCommand(DRAG_DROP_PASTE,
            (files) => {
                (async () => {
                    for (const file of files) {
                        console.log("file", file);
                        if (!file.type.startsWith("image/")) continue;

                        const url = await fileToDataURL(file);
                        editor.dispatchCommand(INSERT_IMAGE_COMMAND, {
                            src: url,
                            align: "left",
                        });
                    }
                })();
                return true;
            },
            COMMAND_PRIORITY_EDITOR)
        return mergeRegister(command, command2);
    }
});

export function $createImageNode(src: string, align: ElementFormatType): ImageNode {
    return new ImageNode(src, align);
}

export function $isImageNode(node: LexicalNode | null): node is ImageNode {
    return node instanceof ImageNode;
}

function fileToDataURL(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result as string);
        reader.onerror = () => reject(reader.error);
        reader.readAsDataURL(file);
    });
}