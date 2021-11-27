mod bf;

use bf::Interpreter;
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let mut args = env::args();
    let process_name = args.next().expect("process name argument should exist");
    let input_file = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} <file_name>", process_name);
        exit(1);
    });
    let file_content = fs::read_to_string(&input_file).unwrap_or_else(|_| {
        eprintln!("[ERROR] Couldn't read input file: {}", input_file);
        exit(1);
    });

    let mut interpreter = Interpreter::new(&file_content);
    interpreter.run();
}
