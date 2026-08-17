import { Level } from "@tiptap/extension-heading";
import Alpine from "alpinejs";
import htmx from "htmx.org";
import "./editor/file";
import "./editor/folder";
import "./StatusBar";
import { create_editor, editor } from "./editor/index";
import { store } from "./editor/store";
import { statusBarStore } from "./StatusBar";

declare global {
	interface Window {
		Alpine: typeof Alpine;
	}
}

htmx.config.noSwap.push(
	...Array.from({ length: 600 - 400 }, (_value, index) => 400 + index),
);

Alpine.data("editor", () => {
	return {
		updatedAt: Date.now(),
		init() {
			create_editor(this.$refs.element, this);

			const id = location.pathname.split("/file/")[1];
			if (id) htmx.ajax("GET", `/api/file/${id}`, { swap: "none" });
		},
		isEmpty() {
			this.updatedAt;
			return editor.isEmpty;
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

window.Alpine = Alpine;
Alpine.start();

htmx.on("htmx:after:request", (e: any) => {
	const { action, method } = e.detail.ctx.request;
	if (method !== "GET" || !action.startsWith("/api/file/")) return;

	const { content } = JSON.parse(e.detail.ctx.text);
	store().set(action.split("/api/file/")[1], content);
});

htmx.on("htmx:before:history:restore", (e: any) => {
	statusBarStore().setUrl(`root${e.detail.path}`);

	const path = location.pathname;
	const mounted = !!document.querySelector('[x-data="editor"]');

	// entrare o uscire dalla sezione note: ripristino normale di htmx
	if (!mounted || (path !== "/" && !path.startsWith("/file/"))) return;

	e.preventDefault();

	const id = path.split("/file/")[1];
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
