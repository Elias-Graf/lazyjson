use lazyjson::treebuilder::node::StringNode;

use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlSpanElement};

use super::ToHtml;

impl ToHtml for StringNode {
    fn to_html(&self, doc: &Document) -> Result<HtmlSpanElement, wasm_bindgen::JsValue> {
        let eml = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;

        eml.set_inner_text(format!("{}", self.val).as_str());

        Ok(eml)
    }
}
