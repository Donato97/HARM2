import Alpine from "alpinejs";

type Mode = "NORMAL" | "INSERT";
type StatusBarStore = {
	mode: Mode;
	url: string;
	error: string;
	modeButton: {};
	init(): void;
	setMode(mode: Mode): void;
	setUrl(newUrl: string): void;
	setError(error: string): void;
};

function hasCaret(el: HTMLElement) {
	const hasContenteditable = () => el.contentEditable === "true";

	switch (el.tagName) {
		case "INPUT":
			return true;
		case "DIV":
			return hasContenteditable();
		default:
			return false;
	}
}

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

	init() {
		document.addEventListener("focusin", (e) => {
			if (hasCaret(e.target as HTMLElement)) {
				this.setMode("INSERT");
			}
		});
		document.addEventListener("focusout", (e) => {
			this.setMode("NORMAL");
		});
	},
	setMode(mode: Mode) {
		this.mode = mode;
	},
	setUrl(newUrl: string) {
		this.url = newUrl;
	},
	setError(error: string) {
		this.error = error;
	},
} satisfies StatusBarStore);

export const statusBarStore = () => Alpine.store("statusBar") as StatusBarStore;
