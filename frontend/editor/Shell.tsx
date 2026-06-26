import { Show } from "solid-js";
import Editor from "./Editor";
import EditorFileSystem from "./filesystem/EditorFileSystem";
import { EFS } from "./filesystem";

export default function Shell() {
    return (
        <main class="drawer md:drawer-open h-full flex">
            <input
                id="file-system-drawer"
                type="checkbox"
                class="drawer-toggle"
            />

            <EditorFileSystem />

            <div class="drawer-content size-full flex flex-col min-h-0 py-20 overflow-y-auto">
                <label
                    for="file-system-drawer"
                    class="btn btn-neutral md:hidden absolute top-4 left-4"
                >
                    <span class="icon-[material-symbols--menu] size-4" />
                </label>

                <Show when={EFS.activeNote() !== undefined}>
                    <Editor />
                </Show>

                <Show when={EFS.activeNote() === undefined}>
                    <p>No active note</p>
                </Show>
            </div>
        </main>
    );
}
