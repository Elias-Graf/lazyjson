use std::{env, fs};

use lazyjson_core::treebuilder::Config;
use lazyjson_emitter_json::EmitJson;

fn main() -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    // let in_file_path = args.get(1).expect("no input file provided");
    let in_file_path = "C:/Users/elias/Desktop/in.json";
    let in_file = fs::read_to_string(in_file_path).expect("failed to read file");

    // let out_file_path = args.get(2).expect("no output file provided");
    let out_file_path = "C:/Users/elias/Desktop/out.json";

    match lazyjson_core::parse(&in_file, &Config::DEFAULT) {
        Err(e) => {
            eprint!("{}", e);
            return Err(1);
        }
        Ok(n) => {
            fs::write(out_file_path, n.unwrap().emit_json(0)).unwrap();
        }
    }

    Ok(())
}
