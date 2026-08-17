use super::models::{Node, Subtree, Tree};
use crate::layouts;
use hypertext::{prelude::*, Raw};

pub struct Props<'a> {
    pub tree: Tree<'a>,
}

pub fn file_button(id: &str, name: &str) -> Raw<String> {
    let url = format!("/file/{id}");
    let api = format!("/api/file/{id}");

    rsx! {
        <button hx-get=(api) hx-push-url=(url) hx-swap="none" class="peer">
            <span class="icon-[material-symbols--notes]"></span>
            <span>(name)</span>
        </button>
    }
    .memoize()
}

pub fn file_markup(file: &Node) -> Raw<String> {
    rsx! {
        <li x-data="file($el)" class="peer" id=(file.id) data-type=(file.type_.as_str())>
            (file_button(&file.id, &file.name))

            <div class="top-0 right-0 absolute opacity-0 hover:opacity-100 peer-hover:opacity-100 p-0">
                <button
                    x-show="!edit"
                    @click="toggleMenu($event, 'file')"
                    class="btn btn-sm btn-ghost"
                    style=(format!("anchor-name: --{}", file.id))
                >
                    <span class="size-4 icon-[material-symbols--more-vert]"></span>
                </button>
            </div>
        </li>
    }
    .memoize()
}

pub fn folder_button(name: &str) -> Raw<String> {
    rsx! {
        <button @click="toggleFolder" class="peer">
            <span "x-bind"="folderButtonIcon" class="icon-[material-symbols--folder-outline]" ></span>
            <span class="border-transparent border-b truncate">
                (name)
            </span>
        </button>
    }.memoize()
}

pub fn folder_markup(folder: &Subtree) -> Raw<String> {
    rsx! {
        <li x-data="folder($el)" "x-bind"="root" data-type=(folder.node.type_.as_str()) id=(folder.node.id) class="peer">

            (folder_button(&folder.node.name))

            <ul x-cloak x-show="open">
                @for folder in &folder.folders {
                    (folder_markup(folder))
                }
                @for file in &folder.files {
                    (file_markup(file))
                }
            </ul>

            <div class="top-0 right-0 absolute opacity-0 hover:opacity-100 peer-hover:opacity-100 p-0">
                <button
                    x-show="!edit"
                    @click="toggleMenu($event, 'folder')"
                    class="btn btn-sm btn-ghost"
                    style=(format!("anchor-name: --{}", folder.node.id))
                >
                    <span class="size-4 icon-[material-symbols--more-vert]"></span>
                </button>
            </div>
        </li>
    }
    .memoize()
}

pub fn index(props: Props<'_>) -> Rendered<String> {
    let markup = rsx! {
        <main class="flex h-full drawer md:drawer-open">
            <input
                id="file-system-drawer"
                type="checkbox"
                class="drawer-toggle"
            />

            <div class="h-full overflow-y-auto drawer-side shrink-0 grow-0">
                <label
                    for="file-system-drawer"
                    aria-label="close sidebar"
                    class="w-full drawer-overlay"
                ></label>

                <div class="bg-base-300 border-base-content/20 border-r w-65 h-full">
                    <div class="flex justify-end items-center p-2 border-base-content/20 border-b" x-data>
                        <button
                            class="btn btn-xs btn-ghost"
                            @click="$dispatch('create-root-folder')"
                        >
                            <span class="size-4 icon-[material-symbols--create-new-folder-outline]"></span>
                        </button>
                        <button
                            class="btn btn-xs btn-ghost"
                            @click="$dispatch('open-all')"
                        >
                            <span class="size-4 icon-[material-symbols--expand]"></span>
                        </button>
                        <button
                            class="btn btn-xs btn-ghost"
                            @click="$dispatch('close-all')"
                        >
                            <span class="size-4 icon-[material-symbols--compress]"></span>
                        </button>
                    </div>

                    <ul class="w-full menu"
                        x-data="tree"
                        x-ref="treeMenu"
                        @create-root-folder.window="startCreate('folder')"
                    >
                        @for folder in &props.tree.folders {
                            (folder_markup(folder))
                        }
                        @for file in &props.tree.files {
                            (file_markup(file))
                        }
                    </ul>
                </div>
            </div>

            <div class="flex flex-col min-h-0 size-full overflow-y-auto drawer-content">
                <label
                    for="file-system-drawer"
                    class="md:hidden top-4 left-4 z-1 absolute btn btn-neutral"
                >
                    <span class="size-4 icon-[material-symbols--menu]"></span>
                </label>

                <div x-data="editor" class="flex flex-col min-h-full grow shrink-0" x-cloak>
                    <div
                        x-show="$store.editor.activeId"
                        x-ref="element"
                        class="mx-auto py-14 min-h-full w-full prose prose-lg shrink-0"></div>

                    <p x-show="!$store.editor.activeId" class="opacity-60 m-auto">
                        "Nessuna nota selezionata"
                    </p>
                </div>
            </div>
        </main>
    };

    let layout_props = layouts::default::Props {
        slot: markup,
        path: "".to_string(),
    };

    layouts::default(layout_props)
}
