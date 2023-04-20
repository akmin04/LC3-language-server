use crate::ast::{
    AddAndOpcodeInstructionNodeValue, DirectiveNodeValue, InstructionNodeValue, LiteralOrLabel,
    Node, NodeValue,
};
use crate::tokens::{
    DirectiveTokenValue, FileLoc, OpcodeTokenValue, RegisterTokenValue, Token, TokenValue,
};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, idx: 0 }
    }

    pub fn parse_ast(&mut self) -> Vec<Node> {
        let mut nodes = Vec::<Node>::new();

        while self.idx < self.tokens.len() {
            let token = self.tokens[self.idx].clone();
            self.idx += 1;

            nodes.push(match &token.value {
                TokenValue::NewLine => Node {
                    value: NodeValue::NewLine,
                    start_loc: token.start_loc,
                    end_loc: token.end_loc,
                    errors: None,
                },
                TokenValue::Label(label) => Node {
                    value: NodeValue::Label(label.clone()),
                    start_loc: token.start_loc,
                    end_loc: token.end_loc,
                    errors: None,
                },
                TokenValue::Comment(comment) => Node {
                    value: NodeValue::Comment(comment.clone()),
                    start_loc: token.start_loc,
                    end_loc: token.end_loc,
                    errors: None,
                },
                TokenValue::Opcode(opcode) => {
                    let (args, end_loc) = self.get_args().unwrap_or((Vec::new(), token.end_loc));

                    match self.parse_instruction_node(
                        *opcode,
                        &args,
                        token.start_loc,
                        token.end_loc,
                    ) {
                        Ok(node) => node,
                        Err(msg) => Node {
                            value: NodeValue::Instruction(InstructionNodeValue::Error {
                                opcode: *opcode,
                                args: Some(args),
                            }),
                            start_loc: token.start_loc,
                            end_loc,
                            errors: Some(vec![msg.to_string()]),
                        },
                    }
                }
                TokenValue::Directive(directive) => {
                    let (args, end_loc) = self.get_args().unwrap_or((Vec::new(), token.end_loc));
                    match self.parse_directive_node(
                        directive.clone(),
                        &args,
                        token.start_loc,
                        token.end_loc,
                    ) {
                        Ok(node) => node,
                        Err(msg) => Node {
                            value: NodeValue::Directive(DirectiveNodeValue::Error {
                                directive: directive.clone(),
                                args: Some(args),
                            }),
                            start_loc: token.start_loc,
                            end_loc,
                            errors: Some(vec![msg.to_string()]),
                        },
                    }
                }
                TokenValue::TrapRoutine(routine) => Node {
                    value: NodeValue::TrapRoutine(*routine),
                    start_loc: token.start_loc,
                    end_loc: token.end_loc,
                    errors: None,
                },
                _ => Node {
                    value: NodeValue::UnexpectedToken(token.clone()),
                    start_loc: token.start_loc,
                    end_loc: token.end_loc,
                    errors: Some(vec!["Unexpected token".to_string()]),
                },
            });
        }

        return nodes;
    }

    fn parse_instruction_node(
        &mut self,
        opcode: OpcodeTokenValue,
        args: &[Token],
        token_start_loc: FileLoc,
        token_end_loc: FileLoc,
    ) -> Result<Node, &'static str> {
        fn and_add_args(args: &[Token]) -> Result<AddAndOpcodeInstructionNodeValue, &'static str> {
            if args.len() != 3 {
                return Err("Incorrect number of arguments (expected 3)");
            }

            if let TokenValue::Register(dr) = &args[0].value {
                if let TokenValue::Register(sr1) = &args[1].value {
                    match &args[2].value {
                        TokenValue::Register(sr2) => {
                            return Ok(AddAndOpcodeInstructionNodeValue::SR2 {
                                dr: *dr,
                                sr1: *sr1,
                                sr2: *sr2,
                            })
                        }
                        TokenValue::NumberLiteral(imm5) => {
                            return Ok(AddAndOpcodeInstructionNodeValue::IMM {
                                dr: *dr,
                                sr1: *sr1,
                                imm5: imm5.clone(),
                            })
                        }
                        _ => {}
                    };
                }
            }

            return Err("Incorrect argument types (expected register, register, register/literal)");
        }

        fn br_jsr_args(args: &[Token]) -> Result<LiteralOrLabel, &'static str> {
            if args.len() != 1 {
                return Err("Incorrect number of arguments (expected 1)");
            }

            return match &args[0].value {
                TokenValue::NumberLiteral(literal) => Ok(LiteralOrLabel::Literal(literal.clone())),
                TokenValue::Label(label) => Ok(LiteralOrLabel::Label(label.clone())),
                _ => Err("Expected literal or label"),
            };
        }

        fn ld_ldi_lea_st_sti_args(
            args: &[Token],
        ) -> Result<(RegisterTokenValue, LiteralOrLabel), &'static str> {
            if args.len() != 2 {
                return Err("Incorrect number of arguments (expected 2)");
            }

            if let TokenValue::Register(arg1) = &args[0].value {
                match &args[1].value {
                    TokenValue::NumberLiteral(arg2) => {
                        return Ok((*arg1, LiteralOrLabel::Literal(arg2.clone())))
                    }
                    TokenValue::Label(arg2) => {
                        return Ok((*arg1, LiteralOrLabel::Label(arg2.clone())))
                    }
                    _ => {}
                };
            }

            return Err("Incorrect argument types (expected register, literal/label)");
        }

        fn ldr_str_args(
            args: &[Token],
        ) -> Result<(RegisterTokenValue, RegisterTokenValue, LiteralOrLabel), &'static str>
        {
            if args.len() != 3 {
                return Err("Incorrect number of arguments (expected 3)");
            }

            if let TokenValue::Register(arg1) = &args[0].value {
                if let TokenValue::Register(arg2) = &args[1].value {
                    match &args[2].value {
                        TokenValue::NumberLiteral(arg3) => {
                            return Ok((*arg1, *arg2, LiteralOrLabel::Literal(arg3.clone())))
                        }
                        TokenValue::Label(arg3) => {
                            return Ok((*arg1, *arg2, LiteralOrLabel::Label(arg3.clone())))
                        }
                        _ => {}
                    };
                }
            }

            return Err("Incorrect argument types (expected register, register, literal/label)");
        }

        match opcode {
            OpcodeTokenValue::ADD => {
                let value = and_add_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::ADD(value)),
                    start_loc: token_start_loc,
                    end_loc: args[2].end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::AND => {
                let value = and_add_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::AND(value)),
                    start_loc: token_start_loc,
                    end_loc: args[2].end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::BR { n, z, p } => {
                let value = br_jsr_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::BR {
                        n,
                        z,
                        p,
                        pc_offset9: value,
                    }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::JMP => {
                if args.len() != 1 {
                    Err("Incorrect number of arguments (expected 1)")
                } else {
                    match &args[0].value {
                        TokenValue::Register(register) => Ok(Node {
                            value: NodeValue::Instruction(InstructionNodeValue::JMP {
                                base_r: *register,
                            }),
                            start_loc: token_start_loc,
                            end_loc: args.last().unwrap().end_loc,
                            errors: None,
                        }),
                        _ => Err("Incorrect argument type (expected register)"),
                    }
                }
            }
            OpcodeTokenValue::JSR => {
                let value = br_jsr_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::JSR { pc_offset11: value }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::LD => {
                let value = ld_ldi_lea_st_sti_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::LD {
                        dr: value.0,
                        pc_offset9: value.1,
                    }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::LDI => {
                let value = ld_ldi_lea_st_sti_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::LDI {
                        dr: value.0,
                        pc_offset9: value.1,
                    }),
                    start_loc: token_start_loc,
                    end_loc: token_end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::LDR => {
                let value = ldr_str_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::LDR {
                        dr: value.0,
                        base_r: value.1,
                        offset6: value.2,
                    }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::LEA => {
                let value = ld_ldi_lea_st_sti_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::LEA {
                        dr: value.0,
                        pc_offset9: value.1,
                    }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::NOT => {
                if args.len() != 2 {
                    return Err("Incorrect number of arguments (expected 2)");
                }

                if let TokenValue::Register(dr) = &args[0].value {
                    if let TokenValue::Register(sr) = &args[1].value {
                        return Ok(Node {
                            value: NodeValue::Instruction(InstructionNodeValue::NOT {
                                dr: *dr,
                                sr: *sr,
                            }),
                            start_loc: token_start_loc,
                            end_loc: args.last().unwrap().end_loc,
                            errors: None,
                        });
                    }
                }

                return Err("Incorrect argument types (expected register, register)");
            }
            OpcodeTokenValue::RET => {
                if args.len() != 0 {
                    Err("Incorrect number of arguments (expected 0)")
                } else {
                    Ok(Node {
                        value: NodeValue::Instruction(InstructionNodeValue::RET),
                        start_loc: token_start_loc,
                        end_loc: token_end_loc,
                        errors: None,
                    })
                }
            }
            OpcodeTokenValue::ST => {
                let value = ld_ldi_lea_st_sti_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::ST {
                        sr: value.0,
                        pc_offset9: value.1,
                    }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::STI => {
                let value = ld_ldi_lea_st_sti_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::STI {
                        sr: value.0,
                        pc_offset9: value.1,
                    }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::STR => {
                let value = ldr_str_args(&args)?;
                Ok(Node {
                    value: NodeValue::Instruction(InstructionNodeValue::LDR {
                        dr: value.0,
                        base_r: value.1,
                        offset6: value.2,
                    }),
                    start_loc: token_start_loc,
                    end_loc: args.last().unwrap().end_loc,
                    errors: None,
                })
            }
            OpcodeTokenValue::TRAP => {
                if args.len() != 1 {
                    Err("Incorrect number of arguments (expected 1)")
                } else {
                    match &args[0].value {
                        TokenValue::NumberLiteral(literal) => Ok(Node {
                            value: NodeValue::Instruction(InstructionNodeValue::TRAP {
                                trapvect8: literal.clone(),
                            }),
                            start_loc: token_start_loc,
                            end_loc: args.last().unwrap().end_loc,
                            errors: None,
                        }),
                        _ => Err("Incorrect argument type (expected literal)"),
                    }
                }
            }
        }
    }

    fn parse_directive_node(
        &mut self,
        directive: DirectiveTokenValue,
        args: &[Token],
        token_start_loc: FileLoc,
        token_end_loc: FileLoc,
    ) -> Result<Node, String> {
        match directive {
            DirectiveTokenValue::ORIG => {
                if args.len() != 1 {
                    Err("Incorrect number of arguments (expected 1)".to_string())
                } else {
                    match &args[0].value {
                        TokenValue::NumberLiteral(literal) => Ok(Node {
                            value: NodeValue::Directive(DirectiveNodeValue::ORIG(literal.clone())),
                            start_loc: token_start_loc,
                            end_loc: token_end_loc,
                            errors: None,
                        }),
                        _ => Err("Incorrect argument type (expected literal)".to_string()),
                    }
                }
            }
            DirectiveTokenValue::FILL => {
                if args.len() != 1 {
                    Err("Incorrect number of arguments (expected 1)".to_string())
                } else {
                    match &args[0].value {
                        TokenValue::NumberLiteral(literal) => Ok(Node {
                            value: NodeValue::Directive(DirectiveNodeValue::FILL(literal.clone())),
                            start_loc: token_start_loc,
                            end_loc: token_end_loc,
                            errors: None,
                        }),
                        _ => Err("Incorrect argument type (expected literal)".to_string()),
                    }
                }
            }
            DirectiveTokenValue::BLKW => {
                if args.len() != 1 {
                    Err("Incorrect number of arguments (expected 1)".to_string())
                } else {
                    match &args[0].value {
                        TokenValue::NumberLiteral(literal) => Ok(Node {
                            value: NodeValue::Directive(DirectiveNodeValue::BLKW(literal.clone())),
                            start_loc: token_start_loc,
                            end_loc: token_end_loc,
                            errors: None,
                        }),
                        _ => Err("Incorrect argument type (expected literal)".to_string()),
                    }
                }
            }
            DirectiveTokenValue::STRINGZ => {
                if args.len() != 1 {
                    Err("Incorrect number of arguments (expected 1)".to_string())
                } else {
                    match &args[0].value {
                        TokenValue::StringLiteral(literal) => Ok(Node {
                            value: NodeValue::Directive(DirectiveNodeValue::STRINGZ(
                                literal.clone(),
                            )),
                            start_loc: token_start_loc,
                            end_loc: token_end_loc,
                            errors: None,
                        }),
                        _ => Err("Incorrect argument type (expected literal)".to_string()),
                    }
                }
            }
            DirectiveTokenValue::END => {
                if args.len() != 0 {
                    Err("Incorrect number of arguments (expected 0)".to_string())
                } else {
                    Ok(Node {
                        value: NodeValue::Directive(DirectiveNodeValue::END),
                        start_loc: token_start_loc,
                        end_loc: token_end_loc,
                        errors: None,
                    })
                }
            }
            DirectiveTokenValue::Error(error) => Err(format!("Unknown directive {}", &error)),
        }
    }

    fn get_args(&mut self) -> Option<(Vec<Token>, FileLoc)> {
        let mut args = Vec::<Token>::new();
        let mut end_loc = None;

        while self.idx < self.tokens.len() {
            let token = &self.tokens[self.idx];
            match token.value {
                TokenValue::Register(_)
                | TokenValue::Label(_)
                | TokenValue::NumberLiteral(_)
                | TokenValue::StringLiteral(_) => {
                    args.push(token.clone());
                    end_loc = Some(token.end_loc);
                    self.idx += 1;
                }
                _ => {
                    break;
                }
            }

            let token = &self.tokens[self.idx];
            match token.value {
                TokenValue::Comma => {
                    self.idx += 1;
                }
                _ => {
                    break;
                }
            }
        }

        match end_loc {
            Some(end_loc) => Some((args, end_loc)),
            None => None,
        }
    }
}
