mod lex;
mod parse;
mod compile;
mod interp;

use crate::lex::tokenize;
use crate::parse::parse;
use crate::compile::compile;
use crate::interp::interpret;

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let tokenized = tokenize(&input);
    let parsed = parse(&tokenized).expect("Failed to parse input");
    let compiled = compile(&parsed, 0);
    interpret(compiled);
}
