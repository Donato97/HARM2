import {
  COMMAND_PRIORITY_HIGH,
  KEY_ARROW_DOWN_COMMAND,
  KEY_ARROW_UP_COMMAND,
  KEY_ENTER_COMMAND,
  LexicalEditor,
} from "lexical";
import { For, onCleanup, Show } from "solid-js";
import useSlashMenu from "./useSlashMenu";

type Props = {
  editor: LexicalEditor;
};

export default function SlashMenu(props: Props) {
  const slash_menu = useSlashMenu();

  const removeUpdateListener = props.editor.registerUpdateListener(
    ({ editorState }) => {
      editorState.read(slash_menu.open);
    },
  );
  const removeArrowDownCommand = props.editor.registerCommand(
    KEY_ARROW_DOWN_COMMAND,
    slash_menu.onArrowDown,
    COMMAND_PRIORITY_HIGH,
  );
  const removeArrowUpCommand = props.editor.registerCommand(
    KEY_ARROW_UP_COMMAND,
    slash_menu.onArrowDown,
    COMMAND_PRIORITY_HIGH,
  );
  const removeEnterCommand = props.editor.registerCommand(
    KEY_ENTER_COMMAND,
    slash_menu.onEnterDown,
    COMMAND_PRIORITY_HIGH,
  );

  onCleanup(() => {
    removeUpdateListener();
    removeArrowDownCommand();
    removeArrowUpCommand();
    removeEnterCommand();
  });

  return (
    <Show when={slash_menu.isOpen()}>
      <div
        class="fixed z-50 max-h-200 overflow-y-auto shadow-2xl rounded-box bg-base-300"
        style={{ left: `${slash_menu.pos.x}px`, top: `${slash_menu.pos.y}px` }}
      >
        <ul class="menu">
          <For each={slash_menu.filteredItems()}>
            {(item, index) => (
              <li>
                <span
                  classList={{
                    "menu-active": index() === slash_menu.selectedIndex(),
                  }}
                >
                  <span class={`${item.icon} size-5`}></span>
                  {item.name}
                </span>
              </li>
            )}
          </For>
        </ul>
      </div>
    </Show>
  );
}
