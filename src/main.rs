mod lex;
mod parse;
mod compile;
mod interp;
mod optimize;

use crate::lex::tokenize;
use crate::parse::parse;
use crate::compile::compile;
use crate::interp::interpret;
use crate::optimize::optimize;

use std::fs::File;
use std::io::{self, Read};
use structopt::StructOpt;
use std::path::PathBuf;
use failure::Error;

#[derive(StructOpt, Debug)]
/// A brainfuck interpreter.
struct Opt {
    /// The brainfuck file to interpret.
    /// If not given, brainfuck sources will be read from stdin.
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let opt = Opt::from_args();
    let src = read_source(opt.file)?;
    let tokens = tokenize(&src);
    let parsed = parse(&tokens)?;
    let optimized = optimize(&parsed);
    let instructions = compile(&optimized, 0);
    interpret(&instructions);
    Ok(())
}

fn read_source(file: Option<PathBuf>) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut reader: Box<dyn Read> = match file {
        Some(path) => Box::new(File::open(&path)?),
        None => Box::new(io::stdin())
    };
    reader.read_to_string(&mut s)?;
    Ok(s)
}
