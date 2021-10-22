use lazyjson::treebuilder::DEFAULT_CONFIG;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::{Event, HtmlElement};

mod node_to_html;

use node_to_html::ToHtml;

#[wasm_bindgen]
pub fn run(cont: &HtmlElement) -> Result<(), JsValue> {
    let document = get_document();
    let input = document
        .create_element("textarea")?
        .dyn_into::<web_sys::HtmlTextAreaElement>()?;
    let output = document
        .create_element("pre")?
        .dyn_into::<web_sys::HtmlPreElement>()?;

    output.set_class_name("lazyjson");

    input.style().set_property("flex", "1")?;

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
                output.set_inner_text(e.as_str());
                return;
            }
        };
        let node = match resp {
            Some(n) => n,
            None => {
                output.set_inner_text(format!("no node in consumer response",).as_str());
                return;
            }
        };

        let to_html_result = match node.to_html(&document) {
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

fn get_document() -> Document {
    web_sys::window()
        .expect("could not get window handle")
        .document()
        .expect("could not get document handle")
}

fn parse(inp: &str) -> Result<Option<lazyjson::treebuilder::node::Node>, String> {
    let toks = match lazyjson::tokenizer::tokenize(inp) {
        Ok(tks) => tks,
        Err(e) => return Err(e.msg(inp)),
    };
    let tree = match lazyjson::treebuilder::value_consumer::value_consumer(
        &mut toks.iter().enumerate().peekable(),
        &DEFAULT_CONFIG,
    ) {
        Ok(n) => n,
        Err(e) => return Err(e.msg(&toks, inp)),
    };

    Ok(tree)
}
