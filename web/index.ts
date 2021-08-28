import init, { run } from "@lazyjson";

const { body } = document;

document.documentElement.style.height = body.style.height = "100%";
document.documentElement.style.width = body.style.width = "100%";

body.style.display = "flex";

await init();

run(body);
