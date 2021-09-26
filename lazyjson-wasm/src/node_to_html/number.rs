use lazyjson::treebuilder::node::NumberSpecific;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlSpanElement};

use super::ToHtml;

impl ToHtml for NumberSpecific {
    fn to_html(&self, doc: &Document) -> Result<HtmlSpanElement, wasm_bindgen::JsValue> {
        let elm = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;

        elm.set_class_name("value number");
        elm.set_inner_text(format!("{}", self.val).as_str());

        Ok(elm)
    }
}
