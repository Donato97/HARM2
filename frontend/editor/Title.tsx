import { createEffect } from "solid-js";
import { EFS } from "./filesystem";
import { EFSClient } from "./filesystem/client";

type Props = {
    onEnter: () => void;
};

type InputEventWithTarget = InputEvent & {
    currentTarget: HTMLHeadingElement;
    target: Element;
};

export default function Title(props: Props) {
    let title!: HTMLHeadingElement;

    function onKeyDown(e: KeyboardEvent) {
        if (e.key === "Enter") {
            e.preventDefault();
            title.blur();
            props.onEnter();
        }
    }

    function onblur() {
        const node = EFS.activeNote()?.file;
        if (!node) return;

        EFSClient.update({
            id: node.id,
            name: node.name,
        });
    }

    function onInput(e: InputEventWithTarget) {
        const el = e.currentTarget;
        const text = el.textContent ?? "";
        if (text === "") {
            el.innerHTML = "";
        }

        const note = EFS.activeNote()?.file;
        if (note) {
            note.name = text;
        }
    }

    createEffect(() => {
        const noteName = EFS.activeNote()?.file.name ?? "";
        if (title.textContent !== noteName) {
            title.textContent = noteName;
        }
    });

    return (
        <h1
            ref={title}
            class="text-4xl font-bold px-4"
            classList={{
                "after:content-['New_document'] after:opacity-50":
                    !EFS.activeNote()?.file.name,
            }}
            contentEditable
            oninput={onInput}
            onkeydown={onKeyDown}
            onblur={onblur}
        ></h1>
    );
}
