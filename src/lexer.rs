//! Lexical analyzer that splits the program text into tokens.

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::tokens::{
    DirectiveTokenValue, FileLoc, NumberLiteralFormat, NumberLiteralTokenValue, OpcodeTokenValue,
    RegisterTokenValue, Token, TokenValue, TrapRoutineTokenValue,
};

pub fn analyze(text: &str) -> Vec<Token> {
    let mut raw_data = text.chars().collect::<Vec<_>>().into_iter().peekable();
    let mut start_loc = FileLoc::new();
    let mut end_loc = FileLoc::new();

    let mut tokens = Vec::<Token>::new();

    loop {
        let mut first_char: Option<char> = None;

        for c in raw_data.by_ref() {
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
            start_loc.col += 1;
            end_loc.col += 1;
        }

        match first_char {
            None => {
                break;
            }
            Some(c) if c == '\n' => {
                tokens.push(Token {
                    value: TokenValue::NewLine,
                    start_loc: start_loc,
                    end_loc: end_loc,
                });

                end_loc.next_line();
            }
            Some(c) if c == ',' => {
                tokens.push(Token {
                    value: TokenValue::Comma,
                    start_loc,
                    end_loc,
                });

                end_loc.col += 1;
            }
            Some(c) if c == '"' => {
                let mut s = String::new();
                get_next_char_while(&mut raw_data, &mut end_loc, &mut s, |c| c != '"');
                raw_data.next();

                tokens.push(Token {
                    value: TokenValue::StringLiteral(s),
                    start_loc,
                    end_loc,
                });

                end_loc.col += 2;
            }
            Some(c) if c == ';' => {
                let mut comment = String::new();
                get_next_char_while(&mut raw_data, &mut end_loc, &mut comment, |c| c != '\n');

                tokens.push(Token {
                    value: TokenValue::Comment(comment),
                    start_loc,
                    end_loc,
                });

                end_loc.col += 1;
            }
            Some(c) if c == '#' || c == 'x' => {
                let mut value = String::new();
                get_next_char_while(&mut raw_data, &mut end_loc, &mut value, |c| {
                    !c.is_whitespace() && c != ','
                });

                let format = match c {
                    '#' => NumberLiteralFormat::Decimal,
                    _ => NumberLiteralFormat::Hex,
                };

                tokens.push(Token {
                    value: TokenValue::NumberLiteral(NumberLiteralTokenValue { format, value }),
                    start_loc,
                    end_loc,
                });

                end_loc.col += 1;
            }
            Some(c) if c == '.' => {
                let mut directive = String::new();
                get_next_char_while(&mut raw_data, &mut end_loc, &mut directive, |c| {
                    !c.is_whitespace()
                });
                directive = directive.to_ascii_uppercase();
                let token = match directive.as_str() {
                    "ORIG" => DirectiveTokenValue::ORIG,
                    "FILL" => DirectiveTokenValue::FILL,
                    "BLKW" => DirectiveTokenValue::BLKW,
                    "STRINGZ" => DirectiveTokenValue::STRINGZ,
                    "END" => DirectiveTokenValue::END,
                    _ => DirectiveTokenValue::Error(directive),
                };

                tokens.push(Token {
                    value: TokenValue::Directive(token),
                    start_loc,
                    end_loc,
                });

                end_loc.col += 1;
            }
            Some(c) => 'case: {
                let mut label = c.to_string();
                get_next_char_while(&mut raw_data, &mut end_loc, &mut label, |c| {
                    !c.is_whitespace() && c != ','
                });
                let label_upper = label.to_ascii_uppercase();

                // Special case with BR opcode with optional NZP suffix
                if label_upper.starts_with("BR") {
                    let suffix = &label_upper[2..];
                    if suffix.chars().all(|c| "NZP".contains(c)) {
                        tokens.push(Token {
                            value: TokenValue::Opcode(OpcodeTokenValue::BR {
                                n: suffix.contains('N'),
                                z: suffix.contains('Z'),
                                p: suffix.contains('P'),
                            }),
                            start_loc,
                            end_loc,
                        });

                        end_loc.col += 1;
                    }

                    break 'case;
                }

                let value = match label_upper.as_str() {
                    "ADD" => TokenValue::Opcode(OpcodeTokenValue::ADD),
                    "AND" => TokenValue::Opcode(OpcodeTokenValue::AND),
                    "JMP" => TokenValue::Opcode(OpcodeTokenValue::JMP),
                    "JSR" => TokenValue::Opcode(OpcodeTokenValue::JSR),
                    "LD" => TokenValue::Opcode(OpcodeTokenValue::LD),
                    "LDI" => TokenValue::Opcode(OpcodeTokenValue::LDI),
                    "LDR" => TokenValue::Opcode(OpcodeTokenValue::LDR),
                    "LEA" => TokenValue::Opcode(OpcodeTokenValue::LEA),
                    "NOT" => TokenValue::Opcode(OpcodeTokenValue::NOT),
                    "RET" => TokenValue::Opcode(OpcodeTokenValue::RET),
                    "ST" => TokenValue::Opcode(OpcodeTokenValue::ST),
                    "STI" => TokenValue::Opcode(OpcodeTokenValue::STI),
                    "STR" => TokenValue::Opcode(OpcodeTokenValue::STR),
                    "TRAP" => TokenValue::Opcode(OpcodeTokenValue::TRAP),

                    "GETC" => TokenValue::TrapRoutine(TrapRoutineTokenValue::GETC),
                    "OUT" => TokenValue::TrapRoutine(TrapRoutineTokenValue::OUT),
                    "PUTS" => TokenValue::TrapRoutine(TrapRoutineTokenValue::PUTS),
                    "IN" => TokenValue::TrapRoutine(TrapRoutineTokenValue::IN),
                    "PUTSP" => TokenValue::TrapRoutine(TrapRoutineTokenValue::PUTSP),
                    "HALT" => TokenValue::TrapRoutine(TrapRoutineTokenValue::HALT),

                    "R0" => TokenValue::Register(RegisterTokenValue::R0),
                    "R1" => TokenValue::Register(RegisterTokenValue::R1),
                    "R2" => TokenValue::Register(RegisterTokenValue::R2),
                    "R3" => TokenValue::Register(RegisterTokenValue::R3),
                    "R4" => TokenValue::Register(RegisterTokenValue::R4),
                    "R5" => TokenValue::Register(RegisterTokenValue::R5),
                    "R6" => TokenValue::Register(RegisterTokenValue::R6),
                    "R7" => TokenValue::Register(RegisterTokenValue::R7),

                    _ => TokenValue::Label(label),
                };

                tokens.push(Token {
                    value,
                    start_loc,
                    end_loc,
                });

                end_loc.col += 1;
            }
        }

        start_loc = end_loc;
    }

    return tokens;
}

fn get_next_char_while(
    raw_data: &mut Peekable<IntoIter<char>>,
    end_loc: &mut FileLoc,
    raw_token: &mut String,
    cond: fn(char) -> bool,
) {
    loop {
        match raw_data.peek() {
            Some(c) if cond(*c) => {
                let c = *c;

                raw_token.push(c);
                raw_data.next();

                if c == '\n' {
                    end_loc.next_line();
                } else {
                    end_loc.col += 1;
                }
            }
            _ => {
                break;
            }
        }
    }
}
