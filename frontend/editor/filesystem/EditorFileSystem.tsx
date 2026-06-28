import { For } from "solid-js";
import { EFS } from ".";
import Folder from "./Folder";



export default function EditorFileSystem() {
    return (
        <div class="drawer-side overflow-y-auto shrink-0 grow-0">
            <label
                for="file-system-drawer"
                aria-label="close sidebar"
                class="drawer-overlay w-full"
            ></label>

            <div class="w-65 bg-base-300 border-r border-base-content/20 h-full">
                <div class="flex items-center justify-end p-2 border-b border-base-content/20">
                    <button
                        class="btn btn-xs btn-ghost"
                        onclick={() => EFS.folder.createRoot()}
                    >
                        <span class="icon-[material-symbols--create-new-folder-outline] size-4" />
                    </button>
                    <button
                        class="btn btn-xs btn-ghost"
                        onclick={() => EFS.openAllFolders(EFS.fileSystem)}
                    >
                        <span class="icon-[material-symbols--expand] size-4" />
                    </button>
                    <button
                        class="btn btn-xs btn-ghost"
                        onclick={() => EFS.closeAllFolders(EFS.fileSystem)}
                    >
                        <span class="icon-[material-symbols--compress] size-4" />
                    </button>
                </div>

                <ul class="menu menu-sm w-full">
                    <For each={Object.entries(EFS.fileSystem)}>
                        {([key, folder]) => (
                            <li>
                                <Folder folder={folder} path={key} />
                            </li>
                        )}
                    </For>
                </ul>
            </div>
        </div>
    );
}