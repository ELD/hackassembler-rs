use std::io::prelude::*;
use std::fs::File;
use std::slice::Iter;

pub struct Lexer {
    lines: Vec<String>,
}

pub struct LexerIter<'a> {
    line_iter: Iter<'a, String>,
}

impl Lexer {
    /// Constructs a new Lexer structure for reading a file and iterating over the lines.
    pub fn new<T: Read>(resource: &mut T) -> Self {
        let mut file_contents = String::new();
        resource.read_to_string(&mut file_contents);
        let lines = file_contents.split('\n').filter_map(|token| if token == "" { None } else { Some(String::from(token)) }).collect::<Vec<String>>();

        Lexer {
            lines: lines,
        }
    }

    pub fn iter<'a>(&'a mut self) -> LexerIter<'a> {
        LexerIter { line_iter: self.lines.iter() }
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.line_iter.next() {
            Some(line) => Some(&line[..]),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, LexerIter};
    use std::io::prelude::*;
    use std::io::Cursor;

    const TEST_INPUT: &'static str = "@2\nD=A\n@3\nD=D+A\n@0\nM=D\n";
    static TEST_TOKENIZED_RESULT: &'static [&'static str] = &["@2", "D=A", "@3", "D=D+A", "@0", "M=D", ""];

    fn setup() -> Lexer {
        Lexer::new(&mut Cursor::new(TEST_INPUT.as_bytes()))
    }

    #[test]
    fn it_tokenizes_input() {
        let lexer = setup();

        assert_eq!(lexer.lines, TEST_TOKENIZED_RESULT);
    }

    #[test]
    fn iter_lines() {
        let mut lexer = setup();
        let mut counter = 0;

        for line in lexer.iter() {
            assert_eq!(line, TEST_TOKENIZED_RESULT[counter]);
            counter += 1;
        }
    }
}

