import { buildEditorFromExtensions } from "@lexical/extension";
import { ListItemNode, ListNode } from "@lexical/list";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { ImageExtension } from "./plugins/image";
import { CodePrismExtension } from "@lexical/code-prism";
import { CodeExtension } from "@lexical/code-core";
import { MermaidExtension } from "./plugins/mermaid";

export const editor = buildEditorFromExtensions({
    name: "[root]",
    namespace: "MyEditor",
    nodes: [HeadingNode, QuoteNode, ListNode, ListItemNode],
    onError: console.error,
    dependencies: [
        ImageExtension,
        CodeExtension,
        CodePrismExtension,
        MermaidExtension
    ],
    theme: {
        code: "editor-code",
        codeHighlight: {
            atrule: "token-attr",
            attr: "token-attr",
            boolean: "token-property",
            builtin: "token-selector",
            cdata: "token-comment",
            char: "token-selector",
            class: "token-function",
            "class-name": "token-function",
            comment: "token-comment",
            constant: "token-property",
            deleted: "token-property",
            doctype: "token-comment",
            entity: "token-operator",
            function: "token-function",
            important: "token-variable",
            inserted: "token-selector",
            keyword: "token-attr",
            namespace: "token-variable",
            number: "token-property",
            operator: "token-operator",
            prolog: "token-comment",
            property: "token-property",
            punctuation: "token-punctuation",
            regex: "token-variable",
            selector: "token-selector",
            string: "token-selector",
            symbol: "token-property",
            tag: "token-property",
            url: "token-operator",
            variable: "token-variable",
        }
    }
});
