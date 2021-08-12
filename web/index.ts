import init, { parse } from "@lazyjson";
import renderNode from "./renderNode";

await init();

const input = document.createElement("textarea");
const output = document.createElement("div");

document.body.append(input);
document.body.append(output);

let inputDebounceTimeout: number;
input.addEventListener("input", () => {
    window.clearTimeout(inputDebounceTimeout);

    inputDebounceTimeout = window.setTimeout(() => {
        while (output.firstChild) output.removeChild(output.firstChild);

        const node = parse(input.value);

        output.append(renderNode(node));
    }, 200);
});
