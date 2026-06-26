import { EditorFile, EditorFileSystem, EditorFolder, EFS } from "./filesystem";

export function _getRoot(): EditorFileSystem {
    return EFS.fileSystem;
}

export function _getFolderNode(path: string): EditorFolder | undefined {
    const pathParts = path.split("/");

    let node: EditorFolder | undefined = EFS.fileSystem[pathParts[0]];
    for (const part of pathParts.slice(1)) {
        node = node?.folders[part];
    }
    return node;
}

export function _getFileNode(path: string): EditorFile | undefined {
    const pathParts = path.split("/");
    const key = path.split("/").at(-1);

    if (!key) return undefined;

    const parentPath = pathParts.slice(0, -1).join("/");
    const parentFolder = _getFolderNode(parentPath);
    return parentFolder?.files[key];
}
