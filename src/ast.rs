//! Data types for the nodes in the LC3 abstract syntax tree.

use crate::tokens::{
    DirectiveTokenValue, FileLoc, NumberLiteralTokenValue, OpcodeTokenValue, RegisterTokenValue,
    Token, TrapRoutineTokenValue,
};

#[derive(Debug)]
pub struct Node {
    pub value: NodeValue,
    pub start_loc: FileLoc,
    pub end_loc: FileLoc,
    pub errors: Vec<NodeError>,
}

#[derive(Debug, Clone)]
pub enum NodeError {
    Error(String),
    Warning(String),
}

#[derive(Debug, Clone)]
pub enum NodeValue {
    NewLine,
    Comment(String),
    Label(String),
    Instruction(InstructionNodeValue),
    Directive(DirectiveNodeValue),
    TrapRoutine(TrapRoutineTokenValue),
    UnexpectedToken(Token),
}

#[derive(Debug, Clone)]
pub enum LiteralOrLabel {
    Literal(NumberLiteralTokenValue),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum InstructionNodeValue {
    ADD(AddAndOpcodeInstructionNodeValue),
    AND(AddAndOpcodeInstructionNodeValue),
    BR {
        n: bool,
        z: bool,
        p: bool,
        pc_offset9: LiteralOrLabel,
    },
    JMP {
        base_r: RegisterTokenValue,
    },
    JSR {
        pc_offset11: LiteralOrLabel,
    },
    // JSRR,
    LD {
        dr: RegisterTokenValue,
        pc_offset9: LiteralOrLabel,
    },
    LDI {
        dr: RegisterTokenValue,
        pc_offset9: LiteralOrLabel,
    },
    LDR {
        dr: RegisterTokenValue,
        base_r: RegisterTokenValue,
        offset6: LiteralOrLabel,
    },
    LEA {
        dr: RegisterTokenValue,
        pc_offset9: LiteralOrLabel,
    },
    NOT {
        dr: RegisterTokenValue,
        sr: RegisterTokenValue,
    },
    RET,
    // RTI,
    ST {
        sr: RegisterTokenValue,
        pc_offset9: LiteralOrLabel,
    },
    STI {
        sr: RegisterTokenValue,
        pc_offset9: LiteralOrLabel,
    },
    STR {
        sr: RegisterTokenValue,
        base_r: RegisterTokenValue,
        offset6: LiteralOrLabel,
    },
    TRAP {
        trapvect8: NumberLiteralTokenValue,
    },
    Error {
        opcode: OpcodeTokenValue,
        args: Option<Vec<Token>>,
    },
}

#[derive(Debug, Clone)]
pub enum AddAndOpcodeInstructionNodeValue {
    SR2 {
        dr: RegisterTokenValue,
        sr1: RegisterTokenValue,
        sr2: RegisterTokenValue,
    },
    IMM {
        dr: RegisterTokenValue,
        sr1: RegisterTokenValue,
        imm5: NumberLiteralTokenValue,
    },
}

#[derive(Debug, Clone)]
pub enum DirectiveNodeValue {
    ORIG(NumberLiteralTokenValue),
    FILL(NumberLiteralTokenValue),
    BLKW(NumberLiteralTokenValue),
    STRINGZ(String),
    END,
    Error {
        directive: DirectiveTokenValue,
        args: Option<Vec<Token>>,
    },
}
