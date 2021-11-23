mod bf;

use bf::BFInterpreter;
use std::fs;

fn main() {
    // TODO: read file passed as argument
    let file_content = fs::read_to_string("test.bf").expect("couldn't read file");
    let interpreter = BFInterpreter::new(&file_content);
}
