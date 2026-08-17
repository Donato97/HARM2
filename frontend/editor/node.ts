import htmx from "htmx.org";
import { html } from "../utils";

type Type = "file" | "folder"

const renameFormHTML = (type: Type) => {
    const folderIcon = `<span class="icon-[material-symbols--folder-outline]" x-bind="folderButtonIcon"></span>`;
    const fileIcon = `<span class="icon-[material-symbols--notes]"></span>`;
    const icon = type === 'folder' ? folderIcon : fileIcon;

    return `
        <div>
            ${icon}
            <input type="text" class="border-primary border-b w-full cursor-text"
                x-init="$el.focus(); $el.select()"
                @keyup.enter="$el.blur()"
                @keyup.escape="cancelRename($el)"
                @blur="confirmRename($el, '${type}')" />
        </div>
    `;
}

const folderMenuHTML = (id: string) => `
    <menu class="menu dropdown bg-base-200 z-1 p-2" popover="auto" style="position-anchor:--${id}">
        <li>
            <button @click="closeMenu(); startCreate('folder')">
                <span class="size-4 icon-[material-symbols--create-new-folder-outline]"></span>
                New folder
            </button>
        </li>
        <li>
            <button @click="closeMenu(); startCreate('file')">
                <span class="size-4 icon-[material-symbols--add-notes]"></span>
                New note
            </button>
        </li>
        <li>
            <button @click="closeMenu(); startRename('folder')">
                <span class="size-4 icon-[material-symbols--edit]"></span>
                Rename
            </button>
        </li>
        <li>
            <button @click="closeMenu(); remove()">
                <span class="size-4 icon-[material-symbols--delete-outline]"></span>
                Delete
            </button>
        </li>
    </menu>
`;

const fileMenuHTML = (id: string) => `
    <menu class="menu dropdown bg-base-200 z-1 p-2" popover="auto" style="position-anchor:--${id}">
        <li>
            <button @click="closeMenu(); startRename('file')">
                <span class="size-4 icon-[material-symbols--edit]"></span>
                Rename
            </button>
        </li>
        <li>
            <button @click="closeMenu(); remove()">
                <span class="size-4 icon-[material-symbols--delete-outline]"></span>
                Delete
            </button>
        </li>
    </menu>
`;

export function node(root: HTMLElement,) {
    let menu: HTMLElement | null = null;
    let renameBackup: HTMLElement | null = null;
    let oldName = '';
    let cancelled = false;

    return {
        edit: false,

        // ---- menu ----
        closeMenu() { menu?.hidePopover() },
        toggleMenu(e: MouseEvent, type: "folder" | "file") {
            if (menu) return menu.hidePopover();

            menu = html(
                type === "folder"
                    ? folderMenuHTML(root.id)
                    : fileMenuHTML(root.id)
            ) as HTMLElement;
            menu.addEventListener('toggle', (ev: ToggleEvent) => {
                if (ev.newState === 'closed') {
                    menu?.remove();
                    menu = null
                }
            });

            (e.currentTarget as HTMLElement).parentElement!.append(menu);

            menu.showPopover();
        },

        // ---- rinomina ----
        startRename(type: Type) {
            this.edit = true;
            cancelled = false;

            const button = root.querySelector(':scope > button') as HTMLElement;
            renameBackup = button;
            oldName = button.textContent!.trim();

            const form = html(renameFormHTML(type));
            form.querySelector('input')!.value = oldName;
            button.replaceWith(form);
        },
        confirmRename(input: HTMLInputElement, type: Type) {
            if (cancelled) return;

            this.edit = false;

            const name = input.value.trim();
            const form = input.closest('div')!;
            if (name === '' || name === oldName) {
                return form.replaceWith(renameBackup!);
            }

            htmx.ajax("PUT", `/filesystem/${root.id}`, {
                values: { name, type_: type },
                target: form,
                swap: "outerHTML",
            });
        },
        cancelRename(input: HTMLElement) {
            cancelled = true;
            this.edit = false;

            input.closest('div')!.replaceWith(renameBackup!);
        },

        // ---- eliminazione ----
        remove() {
            htmx.ajax("DELETE", `/filesystem/${root.id}`, {
                target: root,
                swap: "delete",
            });
        },
    }
}
