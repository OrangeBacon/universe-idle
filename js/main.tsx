import { Backend } from "back-end/back-end";
import { Frontend } from "front-end/front-end";

const root = document.getElementById("app")!;
const front_end = new Frontend(root);
const back_end = new Backend(front_end);
back_end.run();
