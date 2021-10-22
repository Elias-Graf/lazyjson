use treebuilder::{config::Config, node::Node};

pub mod tokenizer;

pub mod treebuilder;

pub mod peak_while;

pub fn parse(inp: &str, config: &Config) -> Result<Option<Node>, String> {
    let toks = match tokenizer::tokenize(inp) {
        Err(e) => return Err(e.msg(inp)),
        Ok(toks) => toks,
    };

    let node = match treebuilder::value_consumer::value_consumer(
        &mut toks.iter().enumerate().peekable(),
        config,
    ) {
        Err(e) => return Err(e.msg(&toks, inp)),
        Ok(node) => node,
    };

    Ok(node)
}
