import { Level } from "@tiptap/extension-heading";
import Alpine from "alpinejs";
import htmx from "htmx.org";
import "./editor/file";
import "./editor/folder";
import "./StatusBar";
import { store } from "./editor/store";
import { statusBarStore } from "./StatusBar";
import { Editor } from "@tiptap/core";
import focus from "@alpinejs/focus";

declare global {
	interface Window {
		Alpine: typeof Alpine;
	}
}

htmx.config.noSwap.push(
	...Array.from({ length: 600 - 400 }, (_value, index) => 400 + index),
);

Alpine.data("editor", () => {
	let editor: Editor;

	return {
		async init() {
			const { getEditor } = await import("./editor/index");
			editor = getEditor()!;

			const id = location.pathname.split("/editor/")[1];
			if (id) htmx.ajax("GET", `/api/file/${id}`, { swap: "none" });
		},
		isLoaded() {
			return editor;
		},
		isActive(type: string, opts = {}) {
			return editor.isActive(type, opts);
		},
		toggleHeading(opts: { level: Level }) {
			editor.chain().toggleHeading(opts).focus().run();
		},
		toggleBold() {
			editor.chain().focus().toggleBold().run();
		},
		toggleItalic() {
			editor.chain().toggleItalic().focus().run();
		},
	};
});

Alpine.plugin(focus);
window.Alpine = Alpine;
Alpine.start();

htmx.on("htmx:after:request", (e: any) => {
	const { action, method } = e.detail.ctx.request;
	if (method !== "GET" || !action.startsWith("/api/file/")) return;

	const { name, content } = JSON.parse(e.detail.ctx.text);
	store().set({ id: action.split("/api/file/")[1], title: name, content });
});

htmx.on("htmx:before:history:restore", (e: any) => {
	statusBarStore().setUrl(`root${e.detail.path}`);

	const path = location.pathname;
	const mounted = !!document.querySelector('[x-data="editor"]');

	// entrare o uscire dalla sezione note: ripristino normale di htmx
	if (!mounted || (path !== "/" && !path.startsWith("/editor/"))) return;

	e.preventDefault();

	const id = path.split("/editor/")[1];
	if (id) {
		return htmx.ajax("GET", `/api/file/${id}`, { swap: "none" });
	}
	store().unset();
});

htmx.on("htmx:before:history:update", (e: any) => {
	statusBarStore().setUrl(`root${e.detail.history.path}`);
});

htmx.on("htmx:response:error", (e) => {
	console.log(e);
});
