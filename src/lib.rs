use std::error::Error;
use treebuilder::node::Node;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Event, HtmlElement, HtmlSpanElement};

pub mod tokenizer;
pub mod treebuilder;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn run(cont: &HtmlElement) -> Result<(), JsValue> {
    let document = get_document();
    let input = document
        .create_element("textarea")?
        .dyn_into::<web_sys::HtmlTextAreaElement>()?;
    let output = document
        .create_element("pre")?
        .dyn_into::<web_sys::HtmlPreElement>()?;

    input.style().set_property("flex", "1")?;

    output.style().set_property("flex", "1")?;
    output.style().set_property("overflow", "scroll")?;

    cont.append_child(&input)?;
    cont.append_child(&output)?;

    let cb = Closure::wrap(Box::new(move |e: Event| {
        let input = e
            .current_target()
            .unwrap()
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .unwrap();

        output.set_inner_html("");

        let result = parse(input.value().as_str());
        let resp = match result {
            Ok(resp) => resp,
            Err(e) => {
                output.set_inner_text(format!("{}", e).as_str());
                return;
            }
        };
        let node = match resp.node {
            Some(n) => n,
            None => {
                output.set_inner_text(
                    format!(
                        "no node in consumer response, consumed {} tokens",
                        resp.cons
                    )
                    .as_str(),
                );
                return;
            }
        };

        let to_html_result = match node.to_html() {
            Ok(element) => output.append_child(&element),
            Err(e) => Err(e),
        };

        if let Err(e) = to_html_result {
            output.set_inner_text(format!("{:?}", e).as_str())
        }
    }) as Box<dyn FnMut(_)>);

    input.add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())?;

    cb.forget();

    Ok(())
}

impl Node {
    pub fn to_html(&self) -> Result<HtmlSpanElement, JsValue> {
        let document = get_document();
        let cont = document
            .create_element("span")?
            .dyn_into::<HtmlSpanElement>()?;
        let node_name = document
            .create_element("span")?
            .dyn_into::<HtmlSpanElement>()?;

        cont.style().set_property("display", "flex")?;

        node_name.set_inner_text(format!("[{}]", self.get_typ_str()).as_str());

        let node_cont = match self {
            Node::Array(a) => {
                let child_cont = document
                    .create_element("span")?
                    .dyn_into::<HtmlSpanElement>()?;

                if a.entries.len() == 0 {
                    child_cont.set_inner_text("<empty>");
                } else {
                    cont.style().set_property("flex-direction", "column")?;
                    child_cont.style().set_property("margin-left", "2em")?;

                    for entry in &a.entries {
                        let child_elm = entry.to_html()?;
                        child_cont.append_child(&child_elm)?;
                    }
                }

                child_cont
            }
            Node::Bool(b) => {
                let val_elm = document
                    .create_element("span")?
                    .dyn_into::<HtmlSpanElement>()?;

                val_elm.set_inner_text(format!("{}", b.val).as_str());

                val_elm
            }
            Node::Null(_) => {
                let val_elm = document
                    .create_element("span")?
                    .dyn_into::<HtmlSpanElement>()?;

                val_elm.set_inner_text("null");

                val_elm
            }
            Node::Number(n) => {
                let val_elm = document
                    .create_element("span")?
                    .dyn_into::<HtmlSpanElement>()?;

                val_elm.set_inner_text(format!("{}", n.val).as_str());

                val_elm
            }
            Node::Object(o) => {
                let entries_cont = document
                    .create_element("span")?
                    .dyn_into::<HtmlSpanElement>()?;

                if o.entries.len() == 0 {
                    entries_cont.set_inner_text("<empty>");
                } else {
                    cont.style().set_property("flex-direction", "column")?;
                    entries_cont.style().set_property("margin-left", "2em")?;

                    for (key, val) in &o.entries {
                        let entry_cont = document
                            .create_element("span")?
                            .dyn_into::<HtmlSpanElement>()?;

                        entry_cont.style().set_property("display", "flex")?;
                        entry_cont.style().set_property("flex-direction", "row")?;

                        let key_elm = document
                            .create_element("span")?
                            .dyn_into::<HtmlSpanElement>()?;
                        let val_elm = val.to_html()?;

                        key_elm.set_inner_text(key.as_str());

                        entry_cont.append_child(&key_elm)?;
                        entry_cont.append_child(&val_elm)?;

                        entries_cont.append_child(&entry_cont)?;
                    }
                }

                entries_cont
            }
            Node::String(s) => {
                let val_elm = document
                    .create_element("span")?
                    .dyn_into::<HtmlSpanElement>()?;

                val_elm.set_inner_text(format!("{}", s.val).as_str());

                val_elm
            }
        };

        cont.append_child(&node_name)?;
        cont.append_child(&node_cont)?;

        let node_debug = format!("{:#?}", self);
        let click_cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            console_log!("{}", node_debug);
        }) as Box<dyn FnMut(_)>);

        node_name.add_event_listener_with_callback("click", &click_cb.as_ref().unchecked_ref())?;

        click_cb.forget();

        Ok(cont)
    }
}

fn get_document() -> Document {
    web_sys::window()
        .expect("could not get window handle")
        .document()
        .expect("could not get document handle")
}

fn parse(inp: &str) -> Result<treebuilder::consumer_response::ConsumerResponse, Box<dyn Error>> {
    let toks = tokenizer::tokenize(inp)?;
    let tree = treebuilder::value_consumer::value_consumer(&toks, 0)?;

    Ok(tree)
}
