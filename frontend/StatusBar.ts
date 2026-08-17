import Alpine from "alpinejs";

type Mode = "NORMAL" | "INSERT";
type StatusBarStore = {
	mode: Mode;
	url: string;
	error: string;
	modeButton: {};
	toggleMode(): Promise<void>;
	setUrl(newUrl: string): void;
	setError(error: string): void;
	resetMode(): Promise<void>;
};

export const statusBarStore = () => Alpine.store("statusBar") as StatusBarStore;

Alpine.store("statusBar", {
	mode: "NORMAL",
	url: `root${window.location.pathname}`,
	error: "",

	modeButton: {
		[":class"]() {
			return {
				"bg-primary text-primary-content":
					statusBarStore().mode === "NORMAL",
				"bg-secondary text-secondary-content":
					statusBarStore().mode === "INSERT",
			};
		},
	},
	async toggleMode() {
		const currentUri = window.location.pathname;
		if (currentUri.includes("/file/")) {
			const { editor } = await import("./editor/index");
			editor.setEditable(!editor.isEditable);
			this.mode = editor.isEditable ? "INSERT" : "NORMAL";
		}
	},
	async resetMode() {
		const currentUri = window.location.pathname;
		if (currentUri.includes("/file/")) {
			const { editor } = await import("./editor/index");
			editor.setEditable(false);
		}

		this.mode = "NORMAL";
	},
	setUrl(newUrl: string) {
		this.url = newUrl;
		this.resetMode();
	},
	setError(error: string) {
		this.error = error;
	},
} satisfies StatusBarStore);
