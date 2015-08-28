extern crate hackasm;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use hackasm::syntax::lex::Lexer;
use hackasm::syntax::parse::Parser;

fn main() {
    let file_path = match env::args().nth(1) {
        Some(file_path) => file_path,
        None => panic!("No file path supplied!"),
    };

    let mut my_file = match File::open(file_path) {
        Ok(file) => file,
        Err(what) => panic!("Something went wrong: {}", Error::description(&what)),
    };

    let mut lexer = Lexer::new(&mut my_file);
    let parser = Parser::new();
    let opcodes = lexer.iter().map(|token| parser.parse(token)).collect::<Vec<String>>();

    for opcode in opcodes.iter() {
        println!("{}", opcode);
    }
}
