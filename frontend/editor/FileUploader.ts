import FileHandler from "@tiptap/extension-file-handler";

export const FileUploader = FileHandler.configure({
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