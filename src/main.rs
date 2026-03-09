use std::env;
use std::fs;


mod instruction;
mod parser;
mod vm;

// use instruction::Instruction;
use parser::parse;
use vm::interpret;

fn read_file(filename: String) -> Vec<u8> {
    let data = fs::read(filename);
    if data.is_err() {
        return Vec::<u8>::new();
    }
    return data
        .ok()
        .unwrap()
        .into_iter()
        .filter(|c| *c == b' ' || *c == b'\t' || *c == b'\n')
        .collect();
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Expected file input");
        return;
    }

    let filename = &args[1];
    let whitespace_source = read_file(filename.to_string());

    let instructions = parse(&whitespace_source);

    interpret(&instructions);

}
