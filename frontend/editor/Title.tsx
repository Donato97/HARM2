import { createSignal } from "solid-js";

type Props = {
  onEnter: () => void;
};

type InputEventWithTarget = InputEvent & {
  currentTarget: HTMLHeadingElement;
  target: Element;
};

export default function Title(props: Props) {
  const [title, setTitle] = createSignal("");

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      props.onEnter();
    }
  }

  function onInput(e: InputEventWithTarget) {
    const el = e.currentTarget;
    const text = el.textContent ?? "";
    if (text === "") {
      el.innerHTML = "";
    }
    setTitle(text);
  }

  return (
    <h1
      class="text-4xl font-bold px-4"
      classList={{
        "after:content-['New_document'] after:opacity-50": title() === "",
      }}
      contentEditable
      onInput={onInput}
      onKeyDown={onKeyDown}
    ></h1>
  );
}
