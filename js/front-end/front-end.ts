import { StartPane } from "front-end/start-pane";

// Interface for managing the rendering of the application.
export class Frontend {
    #root: HTMLElement;

    // The function that is called when an event occurs in the front end.
    handler: null | ((e: Event) => void) = null;

    // Make the front end, to be inserted into the provided element on the page.
    constructor(root: HTMLElement) {
        this.#root = root;
    }

    // switch to rendering a window for the initial user experience, when first
    // loaded.
    showStartPane() {
        StartPane(this);
    }

    // Get the root element to render into.
    get root(): HTMLElement {
        return this.#root;
    }
}