use std::env;
use std::fs;

use lazyjson;
use lazyjson::treebuilder::config::Config;

fn main() -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect("no input file provided");
    let file = fs::read_to_string(file_path).expect("failed to read file");

    let config = match Config::from_iter(&mut args[2..].iter()) {
        Err(e) => {
            eprintln!("{}", e);
            return Err(1);
        }
        Ok(c) => c,
    };

    if let Err(e) = lazyjson::parse(&file, &config) {
        eprintln!("{}", e);
        return Err(1);
    }

    println!("successfully parsed");

    Ok(())
}
