use std::rc::Rc;

use treebuilder::{config::Config, node::Node, VarDict};

pub mod emit;
pub mod peak_while;
pub mod tokenizer;
pub mod treebuilder;

mod char_queue;

pub fn parse(inp: &str, config: &Config) -> Result<Option<Node>, String> {
    let toks = match tokenizer::tokenize(inp, config) {
        Err(e) => return Err(e.msg(inp)),
        Ok(toks) => toks,
    };

    let node = match treebuilder::value_consumer(
        &mut toks.iter().enumerate().peekable(),
        &Rc::new(VarDict::new()),
        config,
    ) {
        Err(e) => return Err(e.msg(&toks, inp)),
        Ok(node) => node,
    };

    Ok(node)
}
