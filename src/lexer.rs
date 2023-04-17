use std::iter::Peekable;
use std::vec::IntoIter;
use std::{fs, io};

use crate::tokens::{
    Directive, FileLoc, NumberLiteral, NumberLiteralFormat, Opcode, Token, TokenValue, TrapRoutine,
};

pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,

    start_loc: FileLoc,
    end_loc: FileLoc,
}

impl Lexer {
    pub fn from_file(file_path: &str) -> io::Result<Self> {
        Ok(Self::from_text(&fs::read_to_string(file_path)?))
    }

    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
            start_loc: FileLoc::new(),
            end_loc: FileLoc::new(),
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
                self.start_loc.col += 1;
                self.end_loc.col += 1;
            }

            match first_char {
                None => {
                    break;
                }
                Some(c) if c == '\n' => {
                    tokens.push(Token {
                        value: TokenValue::NewLine,
                        start_loc: self.start_loc,
                        end_loc: self.end_loc,
                    });

                    self.end_loc.next_line();
                }
                Some(c) if c == ',' => {
                    tokens.push(Token {
                        value: TokenValue::Comma,
                        start_loc: self.start_loc,
                        end_loc: self.end_loc,
                    });

                    self.end_loc.col += 1;
                }
                Some(c) if c == '"' => {
                    let mut s = String::new();
                    self.get_next_char_while(&mut s, |c| c != '"');
                    self.raw_data.next();

                    tokens.push(Token {
                        value: TokenValue::StringLiteral(s),
                        start_loc: self.start_loc,
                        end_loc: self.end_loc,
                    });

                    self.end_loc.col += 1;
                }
                Some(c) if c == ';' => {
                    let mut comment = String::new();
                    self.get_next_char_while(&mut comment, |c| c != '\n');

                    tokens.push(Token {
                        value: TokenValue::Comment(comment),
                        start_loc: self.start_loc,
                        end_loc: self.end_loc,
                    });

                    self.end_loc.col += 1;
                }
                Some(c) if c == '#' || c == 'x' => {
                    let mut value = String::new();
                    self.get_next_char_while(&mut value, |c| !c.is_whitespace() && c != ',');

                    let format = match c {
                        '#' => NumberLiteralFormat::Decimal,
                        _ => NumberLiteralFormat::Hex,
                    };

                    tokens.push(Token {
                        value: TokenValue::NumberLiteral(NumberLiteral { format, value }),
                        start_loc: self.start_loc,
                        end_loc: self.end_loc,
                    });

                    self.end_loc.col += 1;
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

                    tokens.push(Token {
                        value: TokenValue::Directive(token),
                        start_loc: self.start_loc,
                        end_loc: self.end_loc,
                    });

                    self.end_loc.col += 1;
                }
                Some(c) => 'case: {
                    let mut label = c.to_string();
                    self.get_next_char_while(&mut label, |c| !c.is_whitespace() && c != ',');
                    let label_upper = label.to_ascii_uppercase();

                    // Special case with BR opcode with optional NZP suffix
                    if label_upper.starts_with("BR") {
                        let suffix = &label_upper[2..];
                        if suffix.chars().all(|c| "NZP".contains(c)) {
                            tokens.push(Token {
                                value: TokenValue::Opcode(Opcode::BR {
                                    n: suffix.contains('N'),
                                    z: suffix.contains('Z'),
                                    p: suffix.contains('P'),
                                }),
                                start_loc: self.start_loc,
                                end_loc: self.end_loc,
                            });

                            self.end_loc.col += 1;
                        }

                        break 'case;
                    }

                    let value = match label_upper.as_str() {
                        "ADD" => TokenValue::Opcode(Opcode::ADD),
                        "AND" => TokenValue::Opcode(Opcode::AND),
                        "JMP" => TokenValue::Opcode(Opcode::JMP),
                        "JSR" => TokenValue::Opcode(Opcode::JSR),
                        "LD" => TokenValue::Opcode(Opcode::LD),
                        "LDI" => TokenValue::Opcode(Opcode::LDI),
                        "LDR" => TokenValue::Opcode(Opcode::LDR),
                        "LEA" => TokenValue::Opcode(Opcode::LEA),
                        "NOT" => TokenValue::Opcode(Opcode::NOT),
                        "RET" => TokenValue::Opcode(Opcode::RET),
                        "ST" => TokenValue::Opcode(Opcode::ST),
                        "STI" => TokenValue::Opcode(Opcode::STI),
                        "STR" => TokenValue::Opcode(Opcode::STR),
                        "TRAP" => TokenValue::Opcode(Opcode::TRAP),

                        "GETC" => TokenValue::TrapRoutine(TrapRoutine::GETC),
                        "OUT" => TokenValue::TrapRoutine(TrapRoutine::OUT),
                        "PUTS" => TokenValue::TrapRoutine(TrapRoutine::PUTS),
                        "IN" => TokenValue::TrapRoutine(TrapRoutine::IN),
                        "PUTSP" => TokenValue::TrapRoutine(TrapRoutine::PUTSP),
                        "HALT" => TokenValue::TrapRoutine(TrapRoutine::HALT),

                        "R0" => TokenValue::Register(0),
                        "R1" => TokenValue::Register(1),
                        "R2" => TokenValue::Register(2),
                        "R3" => TokenValue::Register(3),
                        "R4" => TokenValue::Register(4),
                        "R5" => TokenValue::Register(5),
                        "R6" => TokenValue::Register(6),
                        "R7" => TokenValue::Register(7),

                        _ => TokenValue::Label(label),
                    };

                    tokens.push(Token {
                        value,
                        start_loc: self.start_loc,
                        end_loc: self.end_loc,
                    });

                    self.end_loc.col += 1;
                }
            }

            self.start_loc = self.end_loc;
        }

        return tokens;
    }

    fn get_next_char_while(&mut self, raw_token: &mut String, cond: fn(char) -> bool) {
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    let c = *c;

                    raw_token.push(c);
                    self.raw_data.next();

                    if c == '\n' {
                        self.end_loc.next_line();
                    } else {
                        self.end_loc.col += 1;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }
}
