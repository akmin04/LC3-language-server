use crate::ast::{InstructionNodeValue, LiteralOrLabel, Node, NodeValue};

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
                    node.errors.push(format!("Undefined label `{}`", label));
                }
            }
        }
    }
}
