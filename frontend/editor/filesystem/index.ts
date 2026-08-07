import { createMutable } from "solid-js/store";
import { NodesClient, NotesClient } from "./client";
import { createEffect, createSignal } from "solid-js";

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

const store = EditorFileSystem();

function EditorFileSystem() {
    const fileSystem = createMutable<EditorFileSystem>({});
    const [activeNote, setActiveNote] = createSignal<
        { path: string; file: EditorFile } | undefined
    >(undefined);

    NodesClient()
        .fetchFileSystem()
        .then((data) => Object.assign(fileSystem, data))
        .then(openFromUrl);

    window.addEventListener("popstate", openFromUrl);

    function openFromUrl() {
        const id = new URLSearchParams(location.search).get("note");
        if (!id) return;

        const path = EFS._buildPath(id);
        if (!path) return;

        const file = _getFileNode(path);
        if (!file) return;
        console.log({ path, file });
        EFS.setActiveNote({ path, file });
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

    function _buildPath(
        id: string,
        folders: Record<string, EditorFolder> = fileSystem,
        prefix = "",
    ): string | undefined {
        for (const folder of Object.values(folders)) {
            const path = prefix ? `${prefix}/${folder.id}` : folder.id;

            if (folder.files[id]) return `${path}/${id}`;

            const found = _buildPath(id, folder.folders, path);
            if (found) return found;
        }
    }

    function _getRoot(): EditorFileSystem {
        return fileSystem;
    }

    function _getFolderNode(path: string): EditorFolder | undefined {
        const pathParts = path.split("/");

        let node: EditorFolder | undefined = fileSystem[pathParts[0]];
        for (const part of pathParts.slice(1)) {
            node = node?.folders[part];
        }
        return node;
    }

    function _getFileNode(path: string): EditorFile | undefined {
        const pathParts = path.split("/");
        const key = path.split("/").at(-1);

        if (!key) return undefined;

        const parentPath = pathParts.slice(0, -1).join("/");
        const parentFolder = _getFolderNode(parentPath);
        return parentFolder?.files[key];
    }

    return {
        fileSystem,
        activeNote,
        setActiveNote,
        openFromUrl,
        openAllFolders,
        closeAllFolders,
        _buildPath,
        _getRoot,
        _getFolderNode,
        _getFileNode,
    };
}

function Folder() {
    function create(path: string) {
        const depth = path.split("/").length;
        if (depth >= 5) return;

        const node = store._getFolderNode(path);
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

    function createRoot() {
        const key = crypto.randomUUID();
        const root = store._getRoot();

        root[key] = {
            id: key,
            name: "",
            open: false,
            editMode: true,
            files: {},
            folders: {},
        };

        document.getElementById("rename-folder")?.focus();
    }

    function toggleOpen(path: string) {
        const node = store._getFolderNode(path);
        if (!node) return;
        node.open = !node.open;
    }

    function toggleEditMode(path: string) {
        const node = store._getFolderNode(path);
        if (!node) return;
        node.editMode = !node.editMode;

        return node.editMode;
    }

    function remove(path: string) {
        const parts = path.split("/");
        const key = parts.at(-1);
        if (!key) return;

        const parentPath = parts.slice(0, -1).join("/");

        if (parentPath === "") {
            const root = store._getRoot();
            delete root[key];
            return;
        }

        const node = store._getFolderNode(parentPath);
        if (!node) return;

        delete node.folders[key];
    }

    return {
        client: NodesClient(),
        create,
        createRoot,
        toggleOpen,
        toggleEditMode,
        remove,
    };
}

function Note() {
    function create(path: string) {
        const node = store._getFolderNode(path);
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

    function toggleEditMode(path: string) {
        const node = store._getFileNode(path);
        if (!node) return;
        node.editMode = !node.editMode;

        return node.editMode;
    }

    function remove(path: string) {
        const parts = path.split("/");
        const key = parts.at(-1);
        if (!key) return;

        const parentPath = parts.slice(0, -1).join("/");
        const node = store._getFolderNode(parentPath);
        if (!node) return;

        delete node.files[key];
    }

    return {
        client: NotesClient(),
        create,
        toggleEditMode,
        remove,
    };
}

export const EFS = {
    ...store,
    folder: Folder(),
    note: Note(),
};
