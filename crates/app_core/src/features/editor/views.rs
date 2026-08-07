use super::models::{Node, Subtree, Tree};
use crate::layouts;
use hypertext::{prelude::*, Raw};

pub struct Props<'a> {
    pub tree: Tree<'a>,
}

pub fn index(props: Props<'_>) -> Rendered<String> {
    fn file_markup(file: &Node) -> Raw<String> {
        rsx! {
            <li><span>(file.name)</span></li>
        }
        .memoize()
    }

    fn folder_markup(folder: &Subtree) -> Raw<String> {
        rsx! {
            <li x-data="{ open: false }" @open-all.window="open = true" @close-all.window="open = false">
                <button @click="open = !open">
                    <span>(folder.node.name)</span>
                </button>

                <ul x-cloak x-show="open">
                    @for f in &folder.folders {
                        (folder_markup(f))
                    }
                    @for file in &folder.files {
                        (file_markup(file))
                    }
                </ul>
            </li>
        }
        .memoize()
    }

    let markup = rsx! {
        <main class="drawer md:drawer-open h-full flex">
            <input
                id="file-system-drawer"
                type="checkbox"
                class="drawer-toggle"
            />

            /* <EditorFileSystem /> */
            <div class="drawer-side overflow-y-auto shrink-0 grow-0">
                <label
                    for="file-system-drawer"
                    aria-label="close sidebar"
                    class="drawer-overlay w-full"
                ></label>

                <div class="w-65 bg-base-300 border-r border-base-content/20 h-full">
                    <div class="flex items-center justify-end p-2 border-b border-base-content/20" x-data>
                        <button
                            class="btn btn-xs btn-ghost"
                            /* onclick={() => EFS.folder.createRoot()} */
                        >
                            <span class="icon-[material-symbols--create-new-folder-outline] size-4"></span>
                        </button>
                        <button
                            class="btn btn-xs btn-ghost"
                            @click="$dispatch('open-all')"
                        >
                            <span class="icon-[material-symbols--expand] size-4"></span>
                        </button>
                        <button
                            class="btn btn-xs btn-ghost"
                            @click="$dispatch('close-all')"
                        >
                            <span class="icon-[material-symbols--compress] size-4"></span>
                        </button>
                    </div>

                    <ul class="menu menu-sm w-full" x-ref="treeMenu">
                        @for folder in &props.tree.folders {
                            (folder_markup(folder))
                        }
                        @for file in &props.tree.files {
                            (file_markup(file))
                        }
                    </ul>
                </div>
            </div>

            <div class="drawer-content size-full flex flex-col min-h-0 overflow-y-auto">
                <label
                    for="file-system-drawer"
                    class="btn btn-neutral md:hidden absolute top-4 left-4 z-99"
                >
                    <span class="icon-[material-symbols--menu] size-4"></span>
                </label>

                /* <Show when={EFS.activeNote() !== undefined}>
                    <Editor />
                </Show>

                <Show when={EFS.activeNote() === undefined}>
                    <p>No active note</p>
                </Show> */
            </div>
        </main>
    };

    let layout_props = layouts::default::Props {
        slot: markup,
        path: "".to_string(),
    };

    layouts::default(layout_props)
}
