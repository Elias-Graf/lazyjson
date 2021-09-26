use lazyjson::treebuilder::node::ArraySpecific;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlSpanElement};

use super::ToHtml;

impl ToHtml for ArraySpecific {
    fn to_html(&self, doc: &Document) -> Result<web_sys::HtmlSpanElement, wasm_bindgen::JsValue> {
        let cont = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;

        if self.entries.len() == 0 {
            cont.set_class_name("value empty");
            cont.set_inner_text("<empty>");

            return Ok(cont);
        }

        cont.set_class_name("value array");

        for ent in &self.entries {
            let ent_elm = ent.to_html(doc)?;

            cont.append_child(&ent_elm)?;
        }

        Ok(cont)
    }
}
