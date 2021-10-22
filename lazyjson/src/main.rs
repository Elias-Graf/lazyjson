use std::env;
use std::error::Error;
use std::fs;

use lazyjson;
use lazyjson::treebuilder::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect("no input file provided");
    let file = fs::read_to_string(file_path).expect("failed to read file");

    let config = Config::from_iter(&mut args[2..].iter())?;

    lazyjson::parse(&file, &config)?;

    println!("successfully parsed");

    Ok(())
}
