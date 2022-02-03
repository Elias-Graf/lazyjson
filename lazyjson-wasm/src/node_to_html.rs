use lazyjson::treebuilder::node::Node;
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
    let name = match node {
        Node::Array(_) => "array",
        Node::Bool(_) => "bool",
        Node::Null(_) => "null",
        Node::Number(_) => "number",
        Node::Object(_) => "object",
        Node::String(_) => "string",
    };

    elm.set_class_name("name");
    elm.set_inner_text(format!("[{}]", name).as_str());

    Ok(elm)
}

fn get_val_elm(node: &Node, doc: &Document) -> Result<HtmlSpanElement, JsValue> {
    match &node {
        Node::Array(a) => a.to_html(doc),
        Node::Bool(b) => b.to_html(doc),
        Node::Null(n) => n.to_html(doc),
        Node::Number(n) => n.to_html(doc),
        Node::Object(o) => o.to_html(doc),
        Node::String(s) => s.to_html(doc),
    }
}
