extern crate libhackasm;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use libhackasm::syntax::lex::Lexer;
use libhackasm::syntax::parse::Parser;

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
    let mut parser = Parser::new();

    // Collect symbols
    for token in lexer.iter() {
        parser.collect_symbols(token);
    }

    // Translate assembly
    let opcodes = lexer.iter().map(|token| parser.parse(token)).collect::<Vec<String>>();

    let output_file = match File::create("output.hack") {
        Ok(file) => file,
        Err(what) => panic!("Something went wrong: {}", Error::description(&what)),
    };

    let mut output_writer = BufWriter::new(output_file);

    for opcode in &opcodes {
        output_writer.write_all(opcode.as_bytes()).unwrap();
    }
}
