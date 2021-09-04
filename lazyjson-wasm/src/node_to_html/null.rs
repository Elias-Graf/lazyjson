use lazyjson::treebuilder::node::NullNode;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlSpanElement};

use super::ToHtml;

impl ToHtml for NullNode {
    fn to_html(&self, doc: &Document) -> Result<HtmlSpanElement, wasm_bindgen::JsValue> {
        let elm = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;

        elm.set_inner_text("null");

        Ok(elm)
    }
}
