import { createMutable } from "solid-js/store";

export type EditorFile = {
	id: string;
	name: string;
	editMode: boolean;
	content: string;
};

export type EditorFolder = {
	id: string;
	name: string;
	open: boolean;
	editMode: boolean;
	files: Record<string, EditorFile>;
	folders: Record<string, EditorFolder>;
};

export type EditorFileSystem = Record<string, EditorFolder>;

function useEditorFileSystem() {
	const fileSystem = createMutable<EditorFileSystem>({
		root: {
			id: crypto.randomUUID(),
			name: "root",
			open: false,
			editMode: false,
			files: {
				pippo: { id: crypto.randomUUID(), name: "pippo", editMode: false, content: "" },
			},
			folders: {
				test: {
					id: crypto.randomUUID(),
					name: "test",
					open: false,
					editMode: false,
					files: {
						pippo: { id: crypto.randomUUID(), name: "pippo", editMode: false, content: "" },
					},
					folders: {
						subfolder: {
							id: crypto.randomUUID(),
							name: "subfolder",
							open: false,
							editMode: false,
							files: { pippo: { id: crypto.randomUUID(), name: "pippo", editMode: false, content: "" } },
							folders: {},
						},
					},
				},
			},
		},
	});

	function openAllFolders(folder: Record<string, EditorFolder>) {
		for (const childFolder of Object.values(folder)) {
			childFolder.open = true;
			openAllFolders(childFolder.folders);
		}
	}

	function closeAllFolders(folder: Record<string, EditorFolder>) {
		for (const childFolder of Object.values(folder)) {
			childFolder.open = false;
			closeAllFolders(childFolder.folders);
		}
	}

	function toggleFolderEditMode(path: string) {
		const node = _getFolderNode(path);
		if (!node) return;
		node.editMode = !node.editMode;
	}

	function toggleFileEditMode(path: string) {
		const node = _getFileNode(path);
		if (!node) return;
		node.editMode = !node.editMode;
	}

	function toggleOpen(path: string) {
		const node = _getFolderNode(path);
		if (!node) return;
		node.open = !node.open;
	}

	function addFolder(path: string) {
		const depth = path.split("/").length;
		if (depth >= 5) return;

		const node = _getFolderNode(path);
		if (!node) return;

		const key = crypto.randomUUID();

		node.open = true;
		node.folders[key] = {
			id: key,
			name: "",
			open: false,
			editMode: true,
			files: {},
			folders: {},
		};
	}

	function addRootFolder() {
		const key = crypto.randomUUID();

		fileSystem[key] = {
			id: key,
			name: "",
			open: false,
			editMode: true,
			files: {},
			folders: {},
		};

		document.getElementById("rename-folder")?.focus();
	}

	function addFile(path: string) {
		const node = _getFolderNode(path);
		if (!node) return;

		const key = crypto.randomUUID();

		node.open = true;
		node.files[key] = {
			id: key,
			name: "",
			editMode: true,
			content: "",
		};
	}

	function removeFolder(path: string) {
		const parts = path.split("/");
		const key = parts.at(-1);
		if (!key) return;

		const parentPath = parts.slice(0, -1).join("/");

		if (parentPath === "") {
			delete fileSystem[key];
			return;
		}

		const node = _getFolderNode(parentPath);
		if (!node) return;

		delete node.folders[key];
	}

	function removeFile(path: string) {
		const parts = path.split("/");
		const key = parts.at(-1);
		if (!key) return;

		const parentPath = parts.slice(0, -1).join("/")
		const node = _getFolderNode(parentPath);
		if (!node) return;

		delete node.files[key];
	}

	function _getFileNode(path: string): EditorFile | undefined {
		const pathParts = path.split("/").slice(1).slice(0, -1);
		const key = path.split("/").at(-1);

		if (!key) return undefined;

		let node: EditorFolder | undefined = fileSystem.root;
		for (const part of pathParts) {
			node = node?.folders[part];
		}
		return node.files[key];
	}

	function _getFolderNode(path: string): EditorFolder | undefined {
		const pathParts = path.split("/");

		let node: EditorFolder | undefined = fileSystem[pathParts[0]];
		for (const part of pathParts.slice(1)) {
			node = node?.folders[part];
		}
		return node;
	}

	return {
		fileSystem,
		addFolder,
		addRootFolder,
		addFile,
		removeFolder,
		removeFile,
		toggleOpen,
		toggleFolderEditMode,
		toggleFileEditMode,
		openAllFolders,
		closeAllFolders
	};
}

export const EFS = useEditorFileSystem();