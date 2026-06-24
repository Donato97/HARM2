import { createMutable } from "solid-js/store";
import { EFSClient } from "./client";
import { createSignal } from "solid-js";

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
    const fileSystem = createMutable<EditorFileSystem>({});
    const [activeNote, setActiveNote] = createSignal<
        { path: string; file: EditorFile } | undefined
    >(undefined);

    _init();

    function _init() {
        EFSClient.fetchFileSystem().then((data) =>
            Object.assign(fileSystem, data),
        );
    }

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

        if (!node.editMode) {
            EFSClient.create({
                id: node.id,
                parent_id: path.split("/").slice(0, -1).at(-1),
                name: node.name,
                type_: "folder",
            });
        }
    }

    function toggleFileEditMode(path: string) {
        const node = _getFileNode(path);
        if (!node) return;
        node.editMode = !node.editMode;

        if (!node.editMode) {
            EFSClient.create({
                id: node.id,
                parent_id: path.split("/").slice(0, -1).at(-1),
                name: node.name,
                type_: "file",
            });
        }
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

        const parentPath = parts.slice(0, -1).join("/");
        const node = _getFolderNode(parentPath);
        if (!node) return;

        delete node.files[key];
    }

    function _getFileNode(path: string): EditorFile | undefined {
        const pathParts = path.split("/");
        const key = path.split("/").at(-1);

        if (!key) return undefined;

        const parentPath = pathParts.slice(0, -1).join("/");
        const parentFolder = _getFolderNode(parentPath);
        return parentFolder?.files[key];
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
        activeNote,
        setActiveNote,

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
        closeAllFolders,
    };
}

export const EFS = useEditorFileSystem();
