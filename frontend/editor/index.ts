import { debounce } from "@solid-primitives/scheduled";
import { Editor, EditorEvents } from "@tiptap/core";
import FileHandler from "@tiptap/extension-file-handler";
import Image from "@tiptap/extension-image";
import StarterKit from "@tiptap/starter-kit";
import { SuggestionMenu } from "./Suggestion";
import { store } from "./store";
import { statusBarStore } from "../StatusBar";

const save = debounce(({ editor }: EditorEvents["update"]) => {
	const id = store().activeId;
	if (!id) return;

	const { content } = editor.getJSON();
	fetch(`/api/file/${id}/save`, {
		headers: { "Content-Type": "application/json" },
		method: "PUT",
		body: JSON.stringify({ content: JSON.stringify(content) }),
	}).then(async (res) => {
		if (res.ok) return statusBarStore().setError("");

		const { error, message } = await res.json();
		statusBarStore().setError(`${error}: ${message}`);
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

export let editor: Editor;

export function create_editor(editorEl: HTMLElement, _this: any) {
	editor = new Editor({
		element: editorEl,
		editable: false,
		extensions: [
			StarterKit,
			Image.configure({ inline: true }),
			FileUploader,
			SuggestionMenu,
		],
		/* onUpdate: save, */
		onCreate({ editor }) {
			_this.updatedAt = Date.now();
		},
		onUpdate(props) {
			save(props);
			_this.updatedAt = Date.now();
		},
		onSelectionUpdate({ editor }) {
			_this.updatedAt = Date.now();
		},
	});
}
