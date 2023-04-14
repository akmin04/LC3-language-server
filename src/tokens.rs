#[derive(Debug)]
pub enum Token {
    NewLine,
    Comma,
    Comment(String),
    NumberLiteral(NumberLiteral),
    StringLiteral(String),
    Directive(Directive),
    Opcode(Opcode),
    TrapRoutine(TrapRoutine),
    Register(u8),
    Label(String), // 1-20 characters, starting with letter, allows underscores?
}

#[derive(Debug)]
pub struct NumberLiteral {
    pub format: NumberLiteralFormat,
    pub value: String,
}

#[derive(Debug)]
pub enum NumberLiteralFormat {
    Hex,
    Decimal,
}


#[derive(Debug)]
pub enum Directive {
    ORIG,
    FILL,
    BLKW,
    STRINGZ,
    END,
    Error(String),
}

#[derive(Debug)]
pub enum Opcode {
    ADD,
    AND,
    BR { n: bool, z: bool, p: bool },
    JMP,
    JSR,
    // JSRR,
    LD,
    LDI,
    LDR,
    LEA,
    NOT,
    RET,
    // RTI,
    ST,
    STI,
    STR,
    TRAP,
}

#[derive(Debug)]
pub enum TrapRoutine {
    GETC,
    OUT,
    PUTS,
    IN,
    PUTSP,
    HALT,
}

// pub enum Opcode {
//     ADD(AddOpcode),
//     AND(AndOpcode),
//     BR {
//         n: bool,
//         z: bool,
//         p: bool,
//         pc_offset9: LiteralOrLabel<9>,
//     },
//     JMP {
//         base_r: Register,
//     },
//     JSR {
//         pc_offset11: LiteralOrLabel<11>,
//     },
//     // JSRR,
//     LD {
//         dr: Register,
//         pc_offset9: LiteralOrLabel<9>,
//     },
//     LDI {
//         dr: Register,
//         pc_offset9: LiteralOrLabel<9>,
//     },
//     LDR {
//         dr: Register,
//         base_r: Register,
//         offset6: LiteralOrLabel<6>,
//     },
//     LEA {
//         dr: Register,
//         pc_offset9: LiteralOrLabel<9>,
//     },
//     NOT {
//         dr: Register,
//         sr: Register,
//     },
//     RET,
//     // RTI,
//     ST {
//         sr: Register,
//         pc_offset9: LiteralOrLabel<9>,
//     },
//     STI {
//         sr: Register,
//         pc_offset9: LiteralOrLabel<9>,
//     },
//     STR {
//         sr: Register,
//         base_r: Register,
//         offset6: LiteralOrLabel<6>,
//     },
//     TRAP {
//         trapvect8: Literal<8, false>,
//     },
// }

// pub enum AddOpcode {
//     SR2 {
//         dr: Register,
//         sr1: Register,
//         sr2: Register,
//     },
//     IMM {
//         dr: Register,
//         sr2: Register,
//         imm5: Literal<5, false>,
//     },
// }

// pub enum AndOpcode {
//     SR2 {
//         dr: Register,
//         sr1: Register,
//         sr2: Register,
//     },
//     IMM {
//         dr: Register,
//         sr2: Register,
//         imm5: Literal<5, true>,
//     },
// }

// pub enum Directive {
//     ORIG { location: Literal<16, true> },
//     FILL { value: Literal<16, true> },
//     BLKW { size: Literal<16, true> },
//     STRINGZ { text: String },
//     END,
// }
