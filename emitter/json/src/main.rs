use std::{fs, env};

use lazyjson_core::treebuilder::Config;

use lazyjson_emitter_json::Emit;

fn main() -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    let in_file_path = args.get(1).expect("no input file provided");
    let in_file = fs::read_to_string(in_file_path).expect("failed to read file");

    let out_file_path = args.get(2).expect("no output file provided");

    match lazyjson_core::parse(&in_file, &Config::DEFAULT) {
        Err(e) => {
            eprint!("{}", e);
            return Err(1);
        },
        Ok(n) => {
            fs::write(out_file_path, n.unwrap().emit(0)).unwrap();
        },
    }

    Ok(())
}
