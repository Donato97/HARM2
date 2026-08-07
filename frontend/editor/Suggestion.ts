import { command } from "@tiptap/core";
import { Extension } from "@tiptap/core";
import Suggestion, { SuggestionProps } from "@tiptap/suggestion";

type Items = SuggestionProps<
    {
        icon: string;
        name: string;
        node: () => void;
    },
    any
>;

export const items = [
    {
        icon: "icon-[material-symbols--format-h1]",
        name: "Heading 1",
        node: () => {},
    },
    {
        icon: "icon-[material-symbols--format-h2]",
        name: "Heading 2",
        node: () => {},
    },
    {
        icon: "icon-[material-symbols--format-h3]",
        name: "Heading 3",
        node: () => {},
    },
    {
        icon: "icon-[material-symbols--format-h4]",
        name: "Heading 4",
        node: () => {},
    },
    {
        icon: "icon-[material-symbols--format-list-numbered]",
        name: "Numbered list",
        node: () => {},
    },
    {
        icon: "icon-[material-symbols--format-list-bulleted]",
        name: "Bullet list",
        node: () => {},
    },
    {
        icon: "icon-[material-symbols--checklist]",
        name: "Check list",
        node: () => {},
    },
    {
        icon: "icon-[material-symbols--code]",
        name: "Code",
        node: () => {},
    },
];

function menuDOM(props: Items, parent: HTMLElement) {
    props.items.forEach((item, idx) => {
        const li = document.createElement("li");
        const button = document.createElement("button");
        button.textContent = item.name;
        button.addEventListener("click", () => props.command({ id: item }));
        li.appendChild(button);
        parent.appendChild(li);
    });
}

export const SuggestionMenu = Extension.create({
    name: "suggestion-menu",
    addProseMirrorPlugins() {
        return [
            Suggestion({
                editor: this.editor,
                char: "@",
                placement: "bottom-start",
                floatingUi: {
                    strategy: "fixed",
                },
                items: ({ query }) => {
                    return items.filter(({ name }) =>
                        name.toLowerCase().includes(query.toLowerCase()),
                    );
                },
                render: () => {
                    let popup: HTMLElement;
                    let current: typeof items = [];
                    let cleanup: (() => void) | undefined;
                    let selectedIdx = 0;

                    const highlight = () => {
                        [...popup.children].forEach((el, i) =>
                            el.classList.toggle(
                                "menu-active",
                                i === selectedIdx,
                            ),
                        );
                        popup.children[selectedIdx]?.scrollIntoView({
                            block: "nearest",
                        });
                    };

                    return {
                        onStart: (props) => {
                            popup = document.createElement("ul");
                            popup.classList.add(
                                "menu",
                                "bg-base-200",
                                "w-64!",
                                "border",
                                "border-primary",
                            );
                            menuDOM(props, popup);
                            document.body.appendChild(popup);
                            cleanup = props.mount(popup);
                            current = props.items;
                            highlight();
                        },
                        onUpdate: (props) => {
                            popup.innerHTML = "";
                            menuDOM(props, popup);
                            current = props.items;
                            highlight();
                        },
                        onExit: () => {
                            cleanup?.();
                            popup?.remove();
                        },
                        onKeyDown: ({ event }) => {
                            const n = current.length;

                            if (event.key === "ArrowDown") {
                                selectedIdx = (selectedIdx + 1) % n;
                                highlight();
                                return true;
                            }
                            if (event.key === "ArrowUp") {
                                selectedIdx = (selectedIdx - 1 + n) % n;
                                highlight();
                                return true;
                            }
                            if (event.key === "Enter") {
                                //command(current[selectedIdx].node);
                                return true;
                            }

                            return false;
                        },
                    };
                },
            }),
        ];
    },
});
