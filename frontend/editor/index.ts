import { debounce } from "@solid-primitives/scheduled";
import { Editor, EditorEvents } from "@tiptap/core";
import StarterKit from "@tiptap/starter-kit";
import { EFS } from "./filesystem";
import FileHandler from "@tiptap/extension-file-handler";
import Image from "@tiptap/extension-image";

const save = debounce(({ editor }: EditorEvents["update"]) => {
    const id = EFS.activeNote()?.file.id;
    if (!id) return;

    EFS.note.client.update(id, {
        content: JSON.stringify(editor.getJSON()),
    });
}, 1000);

const FileUploader = FileHandler.configure({
    allowedMimeTypes: ["image/png", "image/jpeg", "image/gif", "image/webp"],
    consumePasteEvent: true,
    onDrop: (currentEditor, files, pos) => {
        const formData = new FormData();
        for (const file of files) {
            const fileReader = new FileReader();

            fileReader.readAsArrayBuffer(file);
            fileReader.onload = () => {
                const result = fileReader.result;
                if (!result) return;

                formData.append("file", new Blob([result]));
                fetch("/storage/upload", {
                    method: "POST",
                    body: formData,
                })
                    .then((res) => res.json())
                    .then((data) => {
                        currentEditor
                            .chain()
                            .insertContentAt(pos, {
                                type: "image",
                                attrs: {
                                    src: data.url,
                                },
                            })
                            .focus()
                            .run();
                    });
            };
        }
    },
    onPaste: (currentEditor, files, htmlContent) => {
        const formData = new FormData();
        for (const file of files) {
            const fileReader = new FileReader();

            fileReader.readAsArrayBuffer(file);
            fileReader.onload = () => {
                const result = fileReader.result;
                if (!result) return;

                formData.append("file", new Blob([result]));
                fetch("/storage/upload", {
                    method: "POST",
                    body: formData,
                })
                    .then((res) => res.json())
                    .then((data) => {
                        currentEditor
                            .chain()
                            .insertContentAt(
                                currentEditor.state.selection.anchor,
                                {
                                    type: "image",
                                    attrs: {
                                        src: data.url,
                                    },
                                },
                            )
                            .focus()
                            .run();
                    });
            };
        }
    },
});

export const editor = new Editor({
    element: document.querySelector("#editor"),
    extensions: [StarterKit, Image, FileUploader],
    content: "<p>Hello World!</p>",
    onUpdate: save,
});
