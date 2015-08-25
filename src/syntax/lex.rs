// #TODO:0 Rewrite Lexer using read_to_end and tokenize file into Vec, use Vec to iterate through lines

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Lexer {
    file: BufReader<File>,
    current_line: String,
}

pub struct LexerIter<'a> {
    current_line: Option<&'a str>,
}

impl Lexer {
    /// Constructs a new Lexer structure for reading a file and iterating over the lines.
    pub fn new(file_name: &str) -> Lexer {
        let file = File::open(file_name).unwrap();

        Lexer {
            file: BufReader::new(file),
            current_line: String::new(),
        }
    }

    pub fn iter<'a>(&'a mut self) -> LexerIter<'a> {
        let is_next_line = self.next_line();
        LexerIter { current_line: is_next_line }
    }

    pub fn next_line(&mut self) -> Option<&str> {
    //     if !self.current_line.is_empty() {
    //         self.current_line.clear();
    //     }
    //
    //     match self.file.read_line(&mut self.current_line) {
    //         Ok(_) => Some(self.current_line.trim()),
    //         Err(_) => None,
    //     }
        let current_line = self.file.lines().next().take().unwrap();
        return Some("Test");
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_line
    }
}
