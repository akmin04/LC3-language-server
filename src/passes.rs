use crate::ast::{
    AddAndOpcodeInstructionNodeValue, InstructionNodeValue, LiteralOrLabel, Node, NodeError,
    NodeValue,
};
use crate::tokens::{NumberLiteralFormat, NumberLiteralTokenValue};

pub fn verify_labels(ast: &mut [Node]) {
    let mut labels = Vec::<String>::new();

    for node in &*ast {
        match &node.value {
            NodeValue::Label(label) => {
                labels.push(label.clone());
            }
            _ => {}
        }
    }

    let mut usages = Vec::<(&mut Node, LiteralOrLabel)>::new();
    for node in &mut *ast {
        match node.value.clone() {
            NodeValue::Instruction(instruction) => match instruction {
                InstructionNodeValue::BR {
                    n: _,
                    z: _,
                    p: _,
                    pc_offset9,
                } => usages.push((node, pc_offset9.clone())),
                InstructionNodeValue::JSR { pc_offset11 } => {
                    usages.push((node, pc_offset11.clone()))
                }
                InstructionNodeValue::LD { dr: _, pc_offset9 } => {
                    usages.push((node, pc_offset9.clone()))
                }
                InstructionNodeValue::LDI { dr: _, pc_offset9 } => {
                    usages.push((node, pc_offset9.clone()))
                }
                InstructionNodeValue::LDR {
                    dr: _,
                    base_r: _,
                    offset6,
                } => usages.push((node, offset6.clone())),
                InstructionNodeValue::LEA { dr: _, pc_offset9 } => {
                    usages.push((node, pc_offset9.clone()))
                }
                InstructionNodeValue::ST { sr: _, pc_offset9 } => {
                    usages.push((node, pc_offset9.clone()))
                }
                InstructionNodeValue::STI { sr: _, pc_offset9 } => {
                    usages.push((node, pc_offset9.clone()))
                }
                InstructionNodeValue::STR {
                    sr: _,
                    base_r: _,
                    offset6,
                } => usages.push((node, offset6.clone())),
                _ => {}
            },
            _ => {}
        }
    }

    for (node, literal_or_label) in usages {
        match literal_or_label {
            LiteralOrLabel::Literal(_) => {}
            LiteralOrLabel::Label(label) => {
                if !labels.contains(&label) {
                    node.errors
                        .push(NodeError::Error(format!("Undefined label `{}`", label)));
                }
            }
        }
    }
}

pub fn verify_number_literals_within_range(ast: &mut [Node]) {
    for node in &mut *ast {
        match node.value.clone() {
            NodeValue::Instruction(instruction) => match instruction {
                InstructionNodeValue::ADD(v) => match v {
                    AddAndOpcodeInstructionNodeValue::IMM {
                        dr: _,
                        sr1: _,
                        imm5,
                    } => verify_literal_within_range(node, imm5, true, 5),
                    _ => {}
                },
                InstructionNodeValue::AND(v) => match v {
                    AddAndOpcodeInstructionNodeValue::IMM {
                        dr: _,
                        sr1: _,
                        imm5,
                    } => verify_literal_within_range(node, imm5, true, 5),
                    _ => {}
                },
                InstructionNodeValue::BR {
                    n: _,
                    z: _,
                    p: _,
                    pc_offset9,
                } => match pc_offset9 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 9)
                    }
                    _ => {}
                },
                InstructionNodeValue::JSR { pc_offset11 } => match pc_offset11 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 11)
                    }
                    _ => {}
                },
                InstructionNodeValue::TRAP { trapvect8 } => {
                    verify_literal_within_range(node, trapvect8, false, 8)
                }
                InstructionNodeValue::LD { dr: _, pc_offset9 } => match pc_offset9 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 9)
                    }
                    _ => {}
                },
                InstructionNodeValue::LDI { dr: _, pc_offset9 } => match pc_offset9 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 9)
                    }
                    _ => {}
                },
                InstructionNodeValue::LDR {
                    dr: _,
                    base_r: _,
                    offset6,
                } => match offset6 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 6)
                    }
                    _ => {}
                },
                InstructionNodeValue::LEA { dr: _, pc_offset9 } => match pc_offset9 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 9)
                    }
                    _ => {}
                },
                InstructionNodeValue::ST { sr: _, pc_offset9 } => match pc_offset9 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 9)
                    }
                    _ => {}
                },
                InstructionNodeValue::STI { sr: _, pc_offset9 } => match pc_offset9 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 9)
                    }
                    _ => {}
                },
                InstructionNodeValue::STR {
                    sr: _,
                    base_r: _,
                    offset6,
                } => match offset6 {
                    LiteralOrLabel::Literal(literal) => {
                        verify_literal_within_range(node, literal, true, 6)
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}

fn verify_literal_within_range(
    node: &mut Node,
    literal: NumberLiteralTokenValue,
    sign_extend: bool,
    bits: u32,
) {
    let value = match literal.format {
        NumberLiteralFormat::Hex => isize::from_str_radix(&literal.value, 16),
        NumberLiteralFormat::Decimal => isize::from_str_radix(&literal.value, 10),
    }
    .unwrap();

    let min_value = if sign_extend {
        -(2_isize.pow(bits - 1))
    } else {
        0
    };

    let max_value = if sign_extend {
        -min_value - 1
    } else {
        2_isize.pow(bits) - 1
    };

    if value < min_value || value > max_value {
        node.errors.push(NodeError::Warning(format!(
            "Number literal `{}{}` is out of range. Must be within [{}, {}] ({} bits {})",
            match literal.format {
                NumberLiteralFormat::Hex => "x",
                NumberLiteralFormat::Decimal => "#",
            },
            literal.value,
            match literal.format {
                NumberLiteralFormat::Hex => format!(
                    "x{}{:X}",
                    if min_value < 0 { "-" } else { "" },
                    min_value.abs()
                ),
                NumberLiteralFormat::Decimal => format!("#{}", min_value),
            },
            match literal.format {
                NumberLiteralFormat::Hex => format!(
                    "x{}{:X}",
                    if min_value < 0 { "-" } else { "" },
                    max_value.abs()
                ),
                NumberLiteralFormat::Decimal => format!("#{}", max_value),
            },
            bits,
            if sign_extend {
                "sign extended"
            } else {
                "zero extended"
            }
        )));
    }
}
