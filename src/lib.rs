use std::error::Error;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, HtmlElement};

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
    let window = web_sys::window().expect("could not get window handle");
    let document = window.document().expect("could not get document handle");
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

        let tree = parse(input.value().as_str());

        console_log!("{:?}", tree);

        output.set_inner_text(format!("{:?}", tree).as_str());
    }) as Box<dyn FnMut(_)>);

    input.add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())?;

    cb.forget();

    Ok(())
}

fn parse(inp: &str) -> Result<treebuilder::consumer_response::ConsumerResponse, Box<dyn Error>> {
    let toks = tokenizer::tokenize(inp)?;
    let tree = treebuilder::value_consumer::value_consumer(&toks, 0)?;

    Ok(tree)
}
