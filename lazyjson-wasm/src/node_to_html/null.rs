use lazyjson::treebuilder::old_node::OldNullNode;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlSpanElement};

use super::ToHtml;

impl ToHtml for OldNullNode {
    fn to_html(&self, doc: &Document) -> Result<HtmlSpanElement, wasm_bindgen::JsValue> {
        let elm = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;

        elm.set_class_name("value null");
        elm.set_inner_text("null");

        Ok(elm)
    }
}
