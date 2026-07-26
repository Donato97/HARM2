import { debounce } from "@solid-primitives/scheduled";
import { Editor, EditorEvents } from "@tiptap/core";
import StarterKit from "@tiptap/starter-kit";
import { EFS } from "./filesystem";

const save = debounce(({ editor }: EditorEvents["update"]) => {
    const id = EFS.activeNote()?.file.id;
    if (!id) return;

    EFS.note.client.update(id, {
        content: JSON.stringify(editor.getJSON()),
    });
}, 1000);

export const editor = new Editor({
    element: document.querySelector("#editor"),
    extensions: [StarterKit],
    content: "<p>Hello World!</p>",
    onUpdate: save,
});
