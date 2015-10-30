use std::collections::HashMap;
use regex::Regex;

pub struct Parser<'a> {
    pc: u32,
    mem: u32,
    l_command_regex: Regex,
    a_command_regex: Regex,
    comp_bits: HashMap<&'a str, &'a str>,
    dest_bits: HashMap<&'a str, &'a str>,
    jump_bits: HashMap<&'a str, &'a str>,
    symbol_table: HashMap<String, u32>,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LCommand,
    ACommand,
    CCommand,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let mut comp_bits = HashMap::new();
        comp_bits.insert("0", "0101010");
        comp_bits.insert("1", "0111111");
        comp_bits.insert("-1", "0111010");
        comp_bits.insert("D", "0001100");
        comp_bits.insert("A", "0110000");
        comp_bits.insert("!D", "0001101");
        comp_bits.insert("!A", "0110001");
        comp_bits.insert("-D", "0001111");
        comp_bits.insert("-A", "0110011");
        comp_bits.insert("D+1", "0011111");
        comp_bits.insert("A+1", "0110111");
        comp_bits.insert("D-1", "0001110");
        comp_bits.insert("A-1", "0110010");
        comp_bits.insert("D+A", "0000010");
        comp_bits.insert("D-A", "0010011");
        comp_bits.insert("A-D", "0000111");
        comp_bits.insert("D&A", "0000000");
        comp_bits.insert("D|A", "0010101");
        comp_bits.insert("M", "1110000");
        comp_bits.insert("!M", "1110001");
        comp_bits.insert("-M", "1110011");
        comp_bits.insert("M+1", "1110111");
        comp_bits.insert("M-1", "1110010");
        comp_bits.insert("D+M", "1000010");
        comp_bits.insert("D-M", "1010011");
        comp_bits.insert("M-D", "1000111");
        comp_bits.insert("D&M", "1000000");
        comp_bits.insert("D|M", "1010101");

        let mut dest_bits = HashMap::new();
        dest_bits.insert("null", "000");
        dest_bits.insert("M", "001");
        dest_bits.insert("D", "010");
        dest_bits.insert("MD", "011");
        dest_bits.insert("A", "100");
        dest_bits.insert("AM", "101");
        dest_bits.insert("AD", "110");
        dest_bits.insert("AMD", "111");

        let mut jump_bits = HashMap::new();
        jump_bits.insert("null", "000");
        jump_bits.insert("JGT", "001");
        jump_bits.insert("JEQ", "010");
        jump_bits.insert("JGE", "011");
        jump_bits.insert("JLT", "100");
        jump_bits.insert("JNE", "101");
        jump_bits.insert("JLE", "110");
        jump_bits.insert("JMP", "111");

        let mut symbol_table = HashMap::new();
        symbol_table.insert(String::from("SP"), 0);
        symbol_table.insert(String::from("LCL"), 1);
        symbol_table.insert(String::from("ARG"), 2);
        symbol_table.insert(String::from("THIS"), 3);
        symbol_table.insert(String::from("THAT"), 4);
        symbol_table.insert(String::from("R0"), 0);
        symbol_table.insert(String::from("R1"), 1);
        symbol_table.insert(String::from("R2"), 2);
        symbol_table.insert(String::from("R3"), 3);
        symbol_table.insert(String::from("R4"), 4);
        symbol_table.insert(String::from("R5"), 5);
        symbol_table.insert(String::from("R6"), 6);
        symbol_table.insert(String::from("R7"), 7);
        symbol_table.insert(String::from("R8"), 8);
        symbol_table.insert(String::from("R9"), 9);
        symbol_table.insert(String::from("R10"), 10);
        symbol_table.insert(String::from("R11"), 11);
        symbol_table.insert(String::from("R12"), 12);
        symbol_table.insert(String::from("R13"), 13);
        symbol_table.insert(String::from("R14"), 14);
        symbol_table.insert(String::from("R15"), 15);
        symbol_table.insert(String::from("SCREEN"), 16384);
        symbol_table.insert(String::from("KBD"), 24576);

        Parser {
            pc: 0,
            mem: 16,
            l_command_regex: Regex::new(r"\((.*)\)").unwrap(),
            a_command_regex: Regex::new(r"@([\w|\d].*)").unwrap(),
            comp_bits: comp_bits,
            dest_bits: dest_bits,
            jump_bits: jump_bits,
            symbol_table: symbol_table,
        }
    }

    pub fn parse<'b>(&'b mut self, token: &'b str) -> String {
        let mut opcode = String::new();
        match self.token_type(token) {
            TokenType::LCommand => {},
            TokenType::ACommand => {
                opcode.push_str("0");
                let capture = self.a_command_regex.captures(token).unwrap();
                let digit = capture.at(1).unwrap();

                let bits: String;
                if self.symbol_table.contains_key(digit) {
                    bits = format!("{:0>15b}", self.symbol_table.get(digit).unwrap());
                } else {
                    match digit.parse::<i32>() {
                        Ok(num) => {
                            bits = format!("{:0>15b}", num);
                        },
                        Err(_) => {
                            if !self.symbol_table.contains_key(digit) {
                                self.symbol_table.insert(String::from(digit), self.mem);
                                self.mem += 1;
                            }
                            bits = format!("{:0>15b}", self.symbol_table.get(digit).unwrap());
                        }
                    };
                }

                opcode = opcode + &bits;
            },
            TokenType::CCommand => {
                opcode.push_str("111");
                let comp_bits = self.get_comp_bits(token);
                let dest_bits = self.get_dest_bits(token);
                let jump_bits = self.get_jump_bits(token);

                opcode = opcode + &comp_bits + &dest_bits + &jump_bits;
            },
        }

        if opcode != "" { opcode.push_str("\n") }
        opcode
    }

    pub fn token_type<'b>(&'b self, token: &'b str) -> TokenType {
        if self.l_command_regex.is_match(token) {
            return TokenType::LCommand;
        }

        if self.a_command_regex.is_match(token) {
            return TokenType::ACommand;
        }

        return TokenType::CCommand;
    }

    pub fn get_comp_bits<'b>(&'b self, token: &'b str) -> String {
        let mut comp_bits = String::new();
        if token.contains(";") {
            let mut comp_split = token.split(";");
            comp_bits = comp_bits + self.comp_bits.get(comp_split.nth(0).unwrap()).unwrap();
        } else {
            let mut comp_split = token.split("=");
            comp_bits = comp_bits + self.comp_bits.get(comp_split.nth(1).unwrap()).unwrap();
        }

        comp_bits
    }

    pub fn get_dest_bits<'b>(&'b self, token: &'b str) -> String {
        if token.contains(";") {
            return String::from(*self.dest_bits.get("null").unwrap());
        }

        let mut dest_split = token.split("=");
        let dest_bits = String::from(*self.dest_bits.get(dest_split.nth(0).unwrap()).unwrap());

        dest_bits
    }

    pub fn get_jump_bits<'b>(&'b self, token: &'b str) -> String {
        if token.contains("=") {
            return String::from(*self.jump_bits.get("null").unwrap());
        }

        let mut jump_split = token.split(";");
        String::from(*self.jump_bits.get(jump_split.nth(1).unwrap()).unwrap())
    }

    pub fn collect_symbols<'b>(&'b mut self, token: &'b str) {
        if token == "" { return; }

        match self.token_type(token) {
            TokenType::LCommand => {
                let capture = self.l_command_regex.captures(token).unwrap();
                let label = capture.at(1).unwrap();
                self.symbol_table.insert(String::from(label), self.pc);
            },
            TokenType::ACommand => {
                self.pc += 1;
            },
            _ => {
                self.pc += 1;
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup<'a>() -> Parser<'a> {
        Parser::new()
    }

    #[test]
    fn create_parser() {
        let parser = setup();
    }

    #[test]
    fn recognizes_token_types() {
        let parser = setup();

        assert_eq!(parser.token_type("(LOOP)"), TokenType::LCommand);
        assert_eq!(parser.token_type("@i"), TokenType::ACommand);
        assert_eq!(parser.token_type("@R2"), TokenType::ACommand);
        assert_eq!(parser.token_type("M=M+D"), TokenType::CCommand);
    }

    #[test]
    fn parse_output_for_a_command() {
        let parser = setup();

        assert_eq!(parser.parse("@5"), "0000000000000101\n");
    }

    #[test]
    fn parse_output_for_c_command() {
        let parser = setup();

        assert_eq!(parser.parse("M=D+A"), "1110000010001000\n");
        assert_eq!(parser.parse("AMD=D|A"), "1110010101111000\n");
        assert_eq!(parser.parse("0;JMP"), "1110101010000111\n");
        assert_eq!(parser.parse("A;JGE"), "1110110000000011\n");
    }

    #[test]
    fn gets_correct_comp_bits() {
        let parser = setup();

        assert_eq!(parser.get_comp_bits("0;JMP"), "0101010");
        assert_eq!(parser.get_comp_bits("M=!A"), "0110001");
        assert_eq!(parser.get_comp_bits("D=!M"), "1110001");
        assert_eq!(parser.get_comp_bits("D&M;JLT"), "1000000");
    }

    #[test]
    fn gets_correct_dest_bits() {
        let parser = setup();

        assert_eq!(parser.get_dest_bits("0;JMP"), "000");
        assert_eq!(parser.get_dest_bits("A=M+D"), "100");
        assert_eq!(parser.get_dest_bits("AM=D|M"), "101");
        assert_eq!(parser.get_dest_bits("MD=D+A"), "011");
        assert_eq!(parser.get_dest_bits("AMD=D&M"), "111");
    }

    #[test]
    fn gets_correct_jump_bits() {
        let parser = setup();

        assert_eq!(parser.get_jump_bits("0;JMP"), "111");
        assert_eq!(parser.get_jump_bits("A=M+D"), "000");
        assert_eq!(parser.get_jump_bits("D;JLE"), "110");
        assert_eq!(parser.get_jump_bits("D|M;JEQ"), "010");
    }

    #[test]
    fn gets_correct_symbols() {
        let parser = setup();

        assert_eq!(*parser.symbol_table.get("R0").unwrap(), 0);
        assert_eq!(*parser.symbol_table.get("SCREEN").unwrap(), 16384);
        assert_eq!(*parser.symbol_table.get("KBD").unwrap(), 24576);
        assert_eq!(*parser.symbol_table.get("LCL").unwrap(), 1);
    }
}
