use lazyjson::treebuilder::node::ObjectNode;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlSpanElement};

use super::ToHtml;

impl ToHtml for ObjectNode {
    fn to_html(&self, doc: &Document) -> Result<HtmlSpanElement, wasm_bindgen::JsValue> {
        let cont = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;

        if self.entries.len() == 0 {
            cont.set_class_name("value empty");
            cont.set_inner_text("<empty>");

            return Ok(cont);
        }

        cont.set_class_name("value object");

        for (key, val) in &self.entries {
            let ent_cont = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;

            ent_cont.set_class_name("entry");

            let key_elm = doc.create_element("span")?.dyn_into::<HtmlSpanElement>()?;
            let val_elm = val.to_html(doc)?;

            key_elm.set_class_name("value string");
            key_elm.set_inner_text(key.as_str());

            ent_cont.append_child(&key_elm)?;
            ent_cont.append_child(&val_elm)?;

            cont.append_child(&ent_cont)?;
        }

        Ok(cont)
    }
}
