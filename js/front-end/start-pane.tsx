import { Frontend } from "front-end/front-end";

export function StartPane(fe: Frontend) {
    const StartButton = "StartPane";

    const pane: DocumentFragment = <>
        <h1>Universe Idle</h1>
        <button id={StartButton}>Start</button>
        <p>This site uses local storage for game data, no tracking is performed</p>
    </>;

    pane.getElementById(StartButton)?.addEventListener("click", e => {
        e.preventDefault();
        fe.handler?.(e);
    });

    fe.root.replaceChildren(pane);
}