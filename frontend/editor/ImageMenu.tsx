import { Portal } from "solid-js/web";

type Props = {
    target: HTMLElement | null;
};

export function ImageMenu(props: Props) {
    return (
        <Portal>
            <div class="absolute z-50" style={props.target ? `left: ${props.target?.getBoundingClientRect()?.left}px; top: ${props.target?.getBoundingClientRect()?.top}px;` : undefined}>
                <p>Image Menu</p>
            </div>
        </Portal>
    );
}