import Alpine from "alpinejs";
import { node } from "./node";

Alpine.data('file', (root: HTMLElement) => {
    return {
        ...node(root)
    };
});