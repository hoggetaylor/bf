mod lex;
mod parse;
mod compile;
mod interp;

use crate::lex::tokenize;
use crate::parse::parse;
use crate::compile::compile;
use crate::interp::interpret;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = env::args().nth(1).expect("Please provide a source file.");
    let mut source = String::new();
    let mut f = File::open(file_name).expect("Could not open file");
    f.read_to_string(&mut source).expect("Failed to read input file");
    let tokenized = tokenize(&source);
    let parsed = parse(&tokenized).expect("Failed to parse input");
    let compiled = compile(&parsed, 0);
    interpret(compiled);
}
