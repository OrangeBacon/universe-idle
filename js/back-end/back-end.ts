import { Frontend } from "front-end/front-end";

export class Backend {
    #fe: Frontend;

    constructor(fe: Frontend) {
        this.#fe = fe;
    }

    run() {
        console.log("Running!");
        this.#fe.handler = (e) => this.startupEvents(e);
        this.#fe.showStartPane();
    }

    startupEvents(e: Event) {
        console.log("start");
    }
}