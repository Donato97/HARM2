import { render } from "solid-js/web";
import Editor from "./editor/Editor";
import "./style.css";

const root = document.getElementById("root")!;
render(() => <Editor />, root);
