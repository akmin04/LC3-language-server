pub enum Token {
    Label(String),
    Opcode(Opcode),
    Directive(Directive),
    Register(Register),
}

pub struct Register {
    number: u8,
}

pub struct Literal<const bits: u8, const sign_extend: bool> {
    format: LiteralFormat,
    value: i16,
}

pub enum LiteralFormat {
    Hex,
    Decimal,
}

pub enum LiteralOrLabel<const bits: u8> {
    Literal(Literal<{ bits }, true>),
    Label(String),
}

pub enum Opcode {
    ADD(AddOpcode),
    AND(AndOpcode),
    BR {
        n: bool,
        z: bool,
        p: bool,
        pc_offset9: LiteralOrLabel<9>,
    },
    JMP {
        base_r: Register,
    },
    JSR {
        pc_offset11: LiteralOrLabel<11>,
    },
    // JSRR,
    LD {
        dr: Register,
        pc_offset9: LiteralOrLabel<9>,
    },
    LDI {
        dr: Register,
        pc_offset9: LiteralOrLabel<9>,
    },
    LDR {
        dr: Register,
        base_r: Register,
        offset6: LiteralOrLabel<6>,
    },
    LEA {
        dr: Register,
        pc_offset9: LiteralOrLabel<9>,
    },
    NOT {
        dr: Register,
        sr: Register,
    },
    RET,
    // RTI,
    ST {
        sr: Register,
        pc_offset9: LiteralOrLabel<9>,
    },
    STI {
        sr: Register,
        pc_offset9: LiteralOrLabel<9>,
    },
    STR {
        sr: Register,
        base_r: Register,
        offset6: LiteralOrLabel<6>,
    },
    TRAP {
        trapvect8: Literal<8, false>,
    },
}

pub enum AddOpcode {
    SR2 {
        dr: Register,
        sr1: Register,
        sr2: Register,
    },
    IMM {
        dr: Register,
        sr2: Register,
        imm5: Literal<5, false>,
    },
}

pub enum AndOpcode {
    SR2 {
        dr: Register,
        sr1: Register,
        sr2: Register,
    },
    IMM {
        dr: Register,
        sr2: Register,
        imm5: Literal<5, true>,
    },
}

pub enum Directive {
    ORIG {
        location: Literal<16, true>,
    },
    FILL {
        value: Literal<16, true>,
    },
    BLKW {
        size: Literal<16, true>,
    },
    STRINGZ {
        text: String,
    },
    END,
}

pub enum TrapRoutine {
    GETC,
    OUT,
    PUTS,
    IN,
    PUTSP,
    HALT,
}
