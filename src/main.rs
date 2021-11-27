mod bf;

use bf::Interpreter;
use std::fs;

fn main() {
    // TODO: read file passed as argument
    let file_content = fs::read_to_string("./data/test.bf").expect("couldn't read file");
    let mut interpreter = Interpreter::new(&file_content);
    interpreter.run();
}
