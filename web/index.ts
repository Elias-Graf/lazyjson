import init, { tokenize } from "../pkg/lazyjson";

await init();

console.log(tokenize("[]"));
