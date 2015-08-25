extern crate hackasm;

use std::env;
use hackasm::syntax::lex::Lexer;

fn main() {
    let file_path = match env::args().nth(1) {
        Some(file_path) => file_path,
        None => panic!("No file path supplied!"),
    };

    let mut lexer = Lexer::new(&file_path);

    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());
    println!("{:?}", lexer.iter().next());

    // Construct parser
    // Pipe (functionally?) each line to codewriter and dump into buffer
    // Flush buffer into file if no failure occurred

    // let mut my_parser = hackasm::parser::Parser::new(&file_path);
    // my_parser.map(|element| {
    //     writer.write(&element);
    // });
}
