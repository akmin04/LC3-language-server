#[derive(Clone, Debug)]
pub struct Token {
    pub value: TokenValue,
    pub start_loc: FileLoc,
    pub end_loc: FileLoc,
}

#[derive(Clone, Copy, Debug)]
pub struct FileLoc {
    pub line: usize,
    pub col: usize,
}

impl FileLoc {
    pub fn new() -> Self {
        FileLoc { line: 1, col: 1 }
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }
}

#[derive(Clone, Debug)]
pub enum TokenValue {
    NewLine,
    Comma,
    Comment(String),
    NumberLiteral(NumberLiteralTokenValue),
    StringLiteral(String),
    Directive(DirectiveTokenValue),
    Opcode(OpcodeTokenValue),
    TrapRoutine(TrapRoutineTokenValue),
    Register(RegisterTokenValue),
    Label(String), // 1-20 characters, starting with letter, allows underscores?
}

#[derive(Clone, Debug)]
pub struct NumberLiteralTokenValue {
    pub format: NumberLiteralFormat,
    pub value: String,
}

#[derive(Clone, Copy, Debug)]
pub enum NumberLiteralFormat {
    Hex,
    Decimal,
}

#[derive(Clone, Debug)]
pub enum DirectiveTokenValue {
    ORIG,
    FILL,
    BLKW,
    STRINGZ,
    END,
    Error(String),
}

#[derive(Clone, Copy, Debug)]
pub enum OpcodeTokenValue {
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

#[derive(Clone, Copy, Debug)]
pub enum TrapRoutineTokenValue {
    GETC,
    OUT,
    PUTS,
    IN,
    PUTSP,
    HALT,
}

#[derive(Clone, Copy, Debug)]
pub enum RegisterTokenValue {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}
