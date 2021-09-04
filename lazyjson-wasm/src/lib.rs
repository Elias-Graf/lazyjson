use std::error::Error;

use node_to_html::ToHtml;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Event, HtmlElement};

mod node_to_html;

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

fn parse(
    inp: &str,
) -> Result<lazyjson::treebuilder::consumer_response::ConsumerResponse, Box<dyn Error>> {
    let toks = lazyjson::tokenizer::tokenize(inp)?;
    let tree = lazyjson::treebuilder::value_consumer::value_consumer(&toks, 0)?;

    Ok(tree)
}
