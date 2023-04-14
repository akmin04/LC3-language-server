use std::iter::Peekable;
use std::vec::IntoIter;
use std::{fs, io};

use crate::tokens::{
    Directive, NumberLiteral, NumberLiteralFormat, Opcode, Token, TrapRoutine,
};

pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
}

impl Lexer {
    pub fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(Self::from_text(&fs::read_to_string(file_path)?))
    }

    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    pub fn analyze(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        loop {
            let mut first_char: Option<char> = None;

            for c in self.raw_data.by_ref() {
                match c {
                    c if c == '\n' => {
                        first_char = Some(c);
                        break;
                    }
                    c if c.is_whitespace() => {}
                    c => {
                        first_char = Some(c);
                        break;
                    }
                }
            }

            match first_char {
                None => {
                    break;
                }
                Some(c) if c == '\n' => {
                    tokens.push(Token::NewLine);
                }
                Some(c) if c == ',' => {
                    tokens.push(Token::Comma);
                }
                Some(c) if c == '"' => {
                    let mut s = String::new();
                    self.get_next_char_while(&mut s, |c| c != '"');
                    self.raw_data.next();
                    tokens.push(Token::StringLiteral(s));
                }
                Some(c) if c == ';' => {
                    let mut comment = String::new();
                    self.get_next_char_while(&mut comment, |c| c != '\n');
                    tokens.push(Token::Comment(comment));
                }
                Some(c) if c == '#' || c == 'x' => {
                    let mut value = String::new();
                    self.get_next_char_while(&mut value, |c| !c.is_whitespace() && c != ',');

                    let format = match c {
                        '#' => NumberLiteralFormat::Decimal,
                        _ => NumberLiteralFormat::Hex,
                    };

                    tokens.push(Token::NumberLiteral(NumberLiteral { format, value }))
                }
                Some(c) if c == '.' => {
                    let mut directive = String::new();
                    self.get_next_char_while(&mut directive, |c| !c.is_whitespace());
                    directive = directive.to_ascii_uppercase();
                    let token = match directive.as_str() {
                        "ORIG" => Directive::ORIG,
                        "FILL" => Directive::FILL,
                        "BLKW" => Directive::BLKW,
                        "STRINGZ" => Directive::STRINGZ,
                        "END" => Directive::END,
                        _ => Directive::Error(directive),
                    };

                    tokens.push(Token::Directive(token));
                }
                Some(c) => {
                    let mut label = c.to_string();
                    self.get_next_char_while(&mut label, |c| !c.is_whitespace() && c != ',');
                    let label_upper = label.to_ascii_uppercase();

                    // Special case with BR opcode with optional NZP suffix
                    if label_upper.starts_with("BR") {
                        let suffix = &label_upper[2..];
                        if suffix.chars().all(|c| "NZP".contains(c)) {
                            tokens.push(Token::Opcode(Opcode::BR {
                                n: suffix.contains('N'),
                                z: suffix.contains('Z'),
                                p: suffix.contains('P'),
                            }));
                        }
                        continue;
                    }

                    let token = match label_upper.as_str() {
                        "ADD" => Token::Opcode(Opcode::ADD),
                        "AND" => Token::Opcode(Opcode::AND),
                        "JMP" => Token::Opcode(Opcode::JMP),
                        "JSR" => Token::Opcode(Opcode::JSR),
                        "LD" => Token::Opcode(Opcode::LD),
                        "LDI" => Token::Opcode(Opcode::LDI),
                        "LDR" => Token::Opcode(Opcode::LDR),
                        "LEA" => Token::Opcode(Opcode::LEA),
                        "NOT" => Token::Opcode(Opcode::NOT),
                        "RET" => Token::Opcode(Opcode::RET),
                        "ST" => Token::Opcode(Opcode::ST),
                        "STI" => Token::Opcode(Opcode::STI),
                        "STR" => Token::Opcode(Opcode::STR),
                        "TRAP" => Token::Opcode(Opcode::TRAP),

                        "GETC" => Token::TrapRoutine(TrapRoutine::GETC),
                        "OUT" => Token::TrapRoutine(TrapRoutine::OUT),
                        "PUTS" => Token::TrapRoutine(TrapRoutine::PUTS),
                        "IN" => Token::TrapRoutine(TrapRoutine::IN),
                        "PUTSP" => Token::TrapRoutine(TrapRoutine::PUTSP),
                        "HALT" => Token::TrapRoutine(TrapRoutine::HALT),

                        "R0" => Token::Register(0),
                        "R1" => Token::Register(1),
                        "R2" => Token::Register(2),
                        "R3" => Token::Register(3),
                        "R4" => Token::Register(4),
                        "R5" => Token::Register(5),
                        "R6" => Token::Register(6),
                        "R7" => Token::Register(7),

                        _ => Token::Label(label),
                    };

                    tokens.push(token);
                }
            }
        }

        return tokens;
    }

    fn get_next_char_while(&mut self, raw_token: &mut String, cond: fn(char) -> bool) {
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    raw_token.push(*c);
                    self.raw_data.next();
                }
                _ => {
                    break;
                }
            }
        }
    }
}
