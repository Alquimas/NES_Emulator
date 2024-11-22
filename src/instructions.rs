use crate::cpu::AddressingMode;

#[derive(Clone)]
pub struct Instruction {
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl Instruction {
    fn new(mnemonic: &'static str, bytes: u8, cycles: u8, mode: AddressingMode) -> Instruction {
        Instruction {
            mnemonic,
            bytes,
            cycles,
            mode,
        }
    }
}

macro_rules! create_instruction {
    ($array:ident, $opcode:expr, $mnemonic:expr, $bytes:expr, $cycles:expr, $mode:expr) => {
        $array[$opcode] = Instruction::new($mnemonic, $bytes, $cycles, $mode);
    };
}

pub fn instruction_table() -> Vec<Instruction> {
    let invalid: Instruction = Instruction::new(
        "XXX",
        0,
        0,
        AddressingMode::Implied
    );
    let mut cpu_instructions: Vec<Instruction> = vec![invalid; 256];

    // LDA
    create_instruction!(cpu_instructions, 0xa9, "LDA", 2, 2,
        AddressingMode::Immediate);
    create_instruction!(cpu_instructions, 0xa5, "LDA", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0xb5, "LDA", 2, 4,
        AddressingMode::ZeroPage_X);
    create_instruction!(cpu_instructions, 0xad, "LDA", 3, 4,
        AddressingMode::Absolute);
    create_instruction!(cpu_instructions, 0xbd, "LDA", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_X);
    create_instruction!(cpu_instructions, 0xb9, "LDA", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_Y);
    create_instruction!(cpu_instructions, 0xa1, "LDA", 2, 6,
        AddressingMode::Indirect_X);
    create_instruction!(cpu_instructions, 0xb1, "LDA", 2, 5,/*+1 if page crossed*/
        AddressingMode::Indirect_Y);

    // LDX
    create_instruction!(cpu_instructions, 0xa2, "LDX", 2, 2,
        AddressingMode::Immediate);
    create_instruction!(cpu_instructions, 0xa6, "LDX", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0xb6, "LDX", 2, 4,
        AddressingMode::ZeroPage_Y);
    create_instruction!(cpu_instructions, 0xae, "LDX", 3, 4,
        AddressingMode::Absolute);
    create_instruction!(cpu_instructions, 0xbe, "LDX", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_Y);

    // LDY
    create_instruction!(cpu_instructions, 0xa0, "LDY", 2, 2,
        AddressingMode::Immediate);
    create_instruction!(cpu_instructions, 0xa4, "LDY", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0xb4, "LDY", 2, 4,
        AddressingMode::ZeroPage_X);
    create_instruction!(cpu_instructions, 0xac, "LDY", 3, 4,
        AddressingMode::Absolute);
    create_instruction!(cpu_instructions, 0xbc, "LDY", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_X);

    // STA
    create_instruction!(cpu_instructions, 0x85, "STA", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0x95, "STA", 2, 4,
        AddressingMode::ZeroPage_X);
    create_instruction!(cpu_instructions, 0x8d, "STA", 3, 4,
        AddressingMode::Absolute);
    create_instruction!(cpu_instructions, 0x9d, "STA", 3, 5,
        AddressingMode::Absolute_X);
    create_instruction!(cpu_instructions, 0x99, "STA", 3, 5,
        AddressingMode::Absolute_Y);
    create_instruction!(cpu_instructions, 0x81, "STA", 2, 6,
        AddressingMode::Indirect_X);
    create_instruction!(cpu_instructions, 0x91, "STA", 2, 6,
        AddressingMode::Indirect_Y);

    // STX
    create_instruction!(cpu_instructions, 0x86, "STX", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0x96, "STX", 2, 4,
        AddressingMode::ZeroPage_Y);
    create_instruction!(cpu_instructions, 0x8e, "STX", 3, 4,
        AddressingMode::Absolute);

    // STY
    create_instruction!(cpu_instructions, 0x84, "STY", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0x94, "STY", 2, 4,
        AddressingMode::ZeroPage_X);
    create_instruction!(cpu_instructions, 0x8c, "STY", 3, 4,
        AddressingMode::Absolute);

    // TAX
    create_instruction!(cpu_instructions, 0xaa, "TAX", 1, 2,
        AddressingMode::Implied);

    // TAY
    create_instruction!(cpu_instructions, 0xa8, "TAY", 1, 2,
        AddressingMode::Implied);

    // TXA
    create_instruction!(cpu_instructions, 0x8a, "TXA", 1, 2,
        AddressingMode::Implied);

    // TYA
    create_instruction!(cpu_instructions, 0x98, "TYA", 1, 2,
        AddressingMode::Implied);

    // TSX
    create_instruction!(cpu_instructions, 0xba, "TSX", 1, 2,
        AddressingMode::Implied);

    // TXS
    create_instruction!(cpu_instructions, 0x9a, "TXS", 1, 2,
        AddressingMode::Implied);

    // PHA
    create_instruction!(cpu_instructions, 0x48, "PHA", 1, 3,
        AddressingMode::Implied);

    // PHP
    create_instruction!(cpu_instructions, 0x08, "PHP", 1, 3,
        AddressingMode::Implied);

    // PLA
    create_instruction!(cpu_instructions, 0x68, "PLA", 1, 4,
        AddressingMode::Implied);

    // PLP
    create_instruction!(cpu_instructions, 0x28, "PLP", 1, 4,
        AddressingMode::Implied);

    // AND
    create_instruction!(cpu_instructions, 0x29, "AND", 2, 2,
        AddressingMode::Immediate);
    create_instruction!(cpu_instructions, 0x25, "AND", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0x35, "AND", 2, 4,
        AddressingMode::ZeroPage_X);
    create_instruction!(cpu_instructions, 0x2d, "AND", 3, 4,
        AddressingMode::Absolute);
    create_instruction!(cpu_instructions, 0x3d, "AND", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_X);
    create_instruction!(cpu_instructions, 0x39, "AND", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_Y);
    create_instruction!(cpu_instructions, 0x21, "AND", 2, 6,
        AddressingMode::Indirect_X);
    create_instruction!(cpu_instructions, 0x31, "AND", 2, 5,/*+1 if page crossed*/
        AddressingMode::Indirect_Y);

    // EOR
    create_instruction!(cpu_instructions, 0x49, "EOR", 2, 2,
        AddressingMode::Immediate);
    create_instruction!(cpu_instructions, 0x45, "EOR", 2, 3,
        AddressingMode::ZeroPage);
    create_instruction!(cpu_instructions, 0x55, "EOR", 2, 4,
        AddressingMode::ZeroPage_X);
    create_instruction!(cpu_instructions, 0x4d, "EOR", 3, 4,
        AddressingMode::Absolute);
    create_instruction!(cpu_instructions, 0x5d, "EOR", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_X);
    create_instruction!(cpu_instructions, 0x59, "EOR", 3, 4,/*+1 if page crossed*/
        AddressingMode::Absolute_Y);
    create_instruction!(cpu_instructions, 0x41, "EOR", 2, 6,
        AddressingMode::Indirect_X);
    create_instruction!(cpu_instructions, 0x51, "EOR", 2, 5,/*+1 if page crossed*/
        AddressingMode::Indirect_Y);

    cpu_instructions
}
