import { run } from "../pkg/lazyjson_wasm";

import "./lazyjson.sass";

const { body } = document;

document.documentElement.style.height = body.style.height = "100%";
document.documentElement.style.width = body.style.width = "100%";

body.style.display = "flex";

run(body);
