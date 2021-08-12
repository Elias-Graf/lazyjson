const BG_COLOR_MAP = new Map([
    ["Array", "blue"],
    ["Bool", "pink"],
    ["Null", "steelblue"],
    ["Number", "lightcoral"],
    ["Object", "red"],
    ["String", "orange"],
]);
const FG_COLOR_MAP = new Map([
    ["Array", "white"],
    ["Bool", "black"],
    ["Null", "black"],
    ["Number", "black"],
    ["Object", "white"],
    ["String", "black"],
]);

// eslint-disable-next-line @typescript-eslint/explicit-module-boundary-types, @typescript-eslint/no-explicit-any
export default function renderNode(node: any): HTMLElement {
    const { typ } = node;
    const ele = document.createElement("span");

    ele.innerText = `[${typ}]`;
    ele.style.display = "inline-block";
    ele.style.color = FG_COLOR_MAP.get(typ) ?? "";
    ele.style.backgroundColor = BG_COLOR_MAP.get(typ) ?? "";

    switch (typ) {
        case "Array": {
            const childrenContainer = document.createElement("span");

            childrenContainer.style.display = "flex";
            childrenContainer.style.flexDirection = "column";
            childrenContainer.style.marginLeft = "1em";
            childrenContainer.append(...node.entries.map(renderNode));

            ele.append(childrenContainer);
            break;
        }
        case "Bool":
            ele.innerText += ` ${node.val}`;
            break;
        case "Number":
            ele.innerText += ` ${node.val}`;
            break;
        case "Object": {
            const childrenContainer = document.createElement("span");

            childrenContainer.style.display = "flex";
            childrenContainer.style.flexDirection = "column";
            childrenContainer.style.marginLeft = "1em";

            Object.entries(node.entries).map(([key, val]) => {
                const child = document.createElement("span");

                child.style.display = "flex";
                child.innerText = `"${key}"`;
                child.append(renderNode(val));

                childrenContainer.append(child);
            });

            ele.append(childrenContainer);
            break;
        }
        case "String":
            ele.innerText += ` "${node.val}"`;
            break;
    }

    return ele;
}
