import Alpine from "alpinejs";
import htmx from "htmx.org";
import { html } from "../utils";
import { node } from "./node";

type Type = "file" | "folder";

const createFormHTML = (type: Type) => {
    const folderIcon = `<span class="icon-[material-symbols--folder-outline]" x-bind="folderButtonIcon"></span>`;
    const fileIcon = `<span class="icon-[material-symbols--notes]"></span>`;
    const icon = type === 'folder' ? folderIcon : fileIcon;

    return `
        <li>
            <div>
                ${icon}
                <input 
                    x-init="$el.focus()" 
                    @keyup.enter="$el.blur()"
                    @keyup.escape="cacelCreation($el)"
                    @blur="create($el, '${type}')" 
                    type="text" 
                    class="border-primary border-b w-full cursor-text" 
                />
            </div>
        </li>
    `;
}

function showForm(type: Type, subtree: Element) {
    Alpine.nextTick(() => {
        const form = html(createFormHTML(type));

        if (type === "folder") {
            const folders = subtree.querySelectorAll(":scope > li[data-type='folder']");
            const last = folders[folders.length - 1];

            if (last) {
                last.insertAdjacentElement('afterend', form);
            } else {
                subtree.insertAdjacentElement('afterbegin', form);
            }
        } else {
            subtree.append(form);
        }
    });
}

function sendRequest(parent_id: string | null, type: Type, input: HTMLInputElement) {
    // il blur arriva prima del keyup.escape: aspetta che l'eventuale annullamento abbia rimosso il form
    Alpine.nextTick(() => {
        if (!input.isConnected) return;

        const name = input.value.trim();
        const li = input.closest('li')!;
        if (name === '') return li.remove();

        htmx.ajax("POST", "/filesystem", {
            values: { id: crypto.randomUUID(), name, type_: type, ...(parent_id && { parent_id }) },
            target: li,
            swap: "outerHTML",
        });
    })
}


Alpine.data('tree', () => {
    return {
        folderButtonIcon: {
            [':class']() { return 'icon-[material-symbols--folder-outline]'; },
        },
        startCreate(type: "file" | "folder") {
            showForm(type, this.$root);
        },
        create(input: HTMLInputElement, type: Type) {
            sendRequest(null, type, input);
        },
        cacelCreation(input: HTMLElement) {
            input.closest('li')?.remove();
        },
    }
})

Alpine.data('folder', (root: HTMLElement) => {
    return {
        open: false,

        root: {
            ['@open-all.window']() { this.open = true },
            ['@close-all.window']() { this.open = false },
        },
        folderButtonIcon: {
            [':class']() {
                return {
                    'icon-[material-symbols--folder-outline]': !this.open,
                    'icon-[material-symbols--folder-open]': this.open,
                };
            },
        },

        toggleFolder() { this.open = !this.open },

        // ---- creazione ----
        startCreate(type: Type) {
            this.open = true;
            const children = this.$root.querySelector(':scope > ul')!;

            showForm(type, children);
        },
        create(input: HTMLInputElement, type: Type) {
            sendRequest(this.$root.id, type, input);
        },
        cacelCreation(input: HTMLElement) {
            input.closest('li')?.remove();
        },

        ...node(root)
    };
});