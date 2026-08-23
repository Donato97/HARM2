import Alpine from "alpinejs";

type SetProps = {
	id: string;
	title: string;
	content: string;
};

type EditorStore = {
	activeId: string | null;
	title: string | null;
	set(props: SetProps): Promise<void>;
	unset(): Promise<void>;
};

const editorStore: EditorStore = {
	title: null as string | null,
	activeId: null as string | null,

	async set({ id, title, content }: SetProps) {
		const { getEditor } = await import("./index");

		this.activeId = id;
		this.title = title;
		getEditor()?.commands.setContent(content ? JSON.parse(content) : "");
	},
	async unset() {
		const { getEditor } = await import("./index");

		this.activeId = null;
		this.title = null;
		getEditor()?.commands.clearContent();
	},
};

Alpine.store("editor", editorStore);

export const store = () => Alpine.store("editor") as EditorStore;
