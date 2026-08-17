export function html(str: string): Element {
    const t = document.createElement('template');
    t.innerHTML = str.trim();
    return t.content.firstElementChild!;
}