use lazyjson::treebuilder::node::{Node, NodeSpecific};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlSpanElement};

pub mod array;
pub mod bool;
pub mod null;
pub mod number;
pub mod object;
pub mod string;

pub trait ToHtml {
    fn to_html(&self, doc: &Document) -> Result<HtmlSpanElement, JsValue>;
}

impl ToHtml for Node {
    fn to_html(&self, doc: &Document) -> Result<HtmlSpanElement, JsValue> {
        let node_elm = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;
        let nam_elm = get_nam_elm(self, doc)?;
        let val_elm = get_val_elm(self, doc)?;

        node_elm.set_class_name("node");
        node_elm.append_child(&nam_elm)?;
        node_elm.append_child(&val_elm)?;

        Ok(node_elm)
    }
}

fn get_nam_elm(node: &Node, doc: &Document) -> Result<HtmlSpanElement, JsValue> {
    let elm = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;
    let name = match node.specific {
        NodeSpecific::Array(_) => "array",
        NodeSpecific::Bool(_) => "bool",
        NodeSpecific::Null(_) => "null",
        NodeSpecific::Number(_) => "number",
        NodeSpecific::Object(_) => "object",
        NodeSpecific::String(_) => "string",
    };

    elm.set_class_name("name");
    elm.set_inner_text(format!("[{}]", name).as_str());

    Ok(elm)
}

fn get_val_elm(node: &Node, doc: &Document) -> Result<HtmlSpanElement, JsValue> {
    match &node.specific {
        NodeSpecific::Array(a) => a.to_html(doc),
        NodeSpecific::Bool(b) => b.to_html(doc),
        NodeSpecific::Null(n) => n.to_html(doc),
        NodeSpecific::Number(n) => n.to_html(doc),
        NodeSpecific::Object(o) => o.to_html(doc),
        NodeSpecific::String(s) => s.to_html(doc),
    }
}
