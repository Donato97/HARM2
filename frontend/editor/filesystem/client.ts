import { EditorFile, EditorFolder } from ".";

export function NodesClient() {
    type CreateBody = {
        id: string;
        name: string;
        parent_id?: string;
        type_: "file" | "folder";
    };

    type UpdateBody = {
        name: string;
    };

    type RawNode = {
        id: string;
        name: string;
        type: "file" | "folder";
        parent_id?: string;
        created_at: string;
        updated_at: string;
    };

    const url = "/api/filesystem";

    async function fetchFileSystem() {
        function _build(node: RawNode): EditorFolder {
            const children = groupedData.get(node.id) || [];
            const files: Record<string, EditorFile> = {};
            const folders: Record<string, EditorFolder> = {};

            for (const child of children) {
                if (child.type === "file") {
                    files[child.id] = {
                        id: child.id,
                        name: child.name,
                        editMode: false,
                        content: "",
                    };
                } else if (child.type === "folder") {
                    folders[child.id] = _build(child);
                }
            }

            return {
                id: node.id,
                name: node.name,
                open: false,
                editMode: false,
                files,
                folders,
            };
        }
        function _groupData() {
            const map = new Map<string, RawNode[]>();
            for (const node of data) {
                const parentId = node.parent_id ?? "root";
                if (!map.has(parentId)) {
                    map.set(parentId, []);
                }
                map.get(parentId)!.push(node);
            }
            return map;
        }

        const data: RawNode[] = await fetch(url, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        }).then((res) => res.json());
        const groupedData = _groupData();
        const fileSystem: Record<string, EditorFolder> = {};
        const rootNodes = groupedData.get("root") ?? [];
        for (const rootNode of rootNodes) {
            if (rootNode.type === "folder") {
                fileSystem[rootNode.id] = _build(rootNode);
            }
        }
        return fileSystem;
    }

    async function createOrUpdate(body: CreateBody) {
        await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        });
    }

    async function update(id: string, body: UpdateBody) {
        await fetch(`${url}/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        });
    }

    async function deleteFolder(id: string) {
        await fetch(`${url}/${id}`, {
            method: "DELETE",
        });
    }

    return {
        fetchFileSystem,
        createOrUpdate,
        update,
        deleteFolder,
    };
}

export function NotesClient() {
    type UpdateBody = {
        content: string;
    };

    const url = "/api/files";

    async function find(id: string) {
        const response = await fetch(`${url}/${id}`, {
            method: "GET",
        });
        const data: { content: string } = await response.json();

        return data.content;
    }

    async function update(id: string, body: UpdateBody) {
        await fetch(`${url}/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        });
    }

    return {
        find,
        update,
    };
}
