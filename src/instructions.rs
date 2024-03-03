pub enum Instruction {
    // Add instructions
    ADD(ArithmeticTarget),
    ADDL(ArithmeticTargetLong),
    ADDI,
    ADC(ArithmeticTarget),
    ADCL(ArithmeticTargetLong),
    ADCI,
    //Subtract Instructions
    SUB(ArithmeticTarget),
    SUBL(ArithmeticTargetLong),
    SUBI,
    SBC(ArithmeticTarget),
    SBCL(ArithmeticTargetLong),
    SBCI,
    //Compare instructions
    CMP(ArithmeticTarget),
    CMPL(ArithmeticTargetLong),
    CMPI,
    //Increment Instructions
    INC(IncDecTarget),
    //Decrement instructions
    DEC(IncDecTarget),
    // Bitwise AND
    AND(ArithmeticTarget),
    ANDL(ArithmeticTargetLong),
    ANDI,
    // Bitwise OR
    OR(ArithmeticTarget),
    ORL(ArithmeticTargetLong),
    ORI,
    // Bitwise XOR
    XOR(ArithmeticTarget),
    XORL(ArithmeticTargetLong),
    XORI,
    //Others
    CCF,
    SCF,
    DAA, /* To be read about */
    CPL,

    // Rotate Instructions
    RLCA,
    RRCA,
    RLA,
    RRA,
    RLC(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RL(ArithmeticTarget),
    RR(ArithmeticTarget),

    // Shift Instructions
    SLA(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    SRL(ArithmeticTarget),

    //Bit instructions
    BIT(u8, ArithmeticTarget),
    RES(u8, ArithmeticTarget),
    SET(u8, ArithmeticTarget),

    JP(JumpTest),
    NOP,
    LD(LoadType),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // Rotate Left Carry Instructions
            0x00 => Some(Instruction::RLC(ArithmeticTarget::B)),
            0x01 => Some(Instruction::RLC(ArithmeticTarget::C)),
            0x02 => Some(Instruction::RLC(ArithmeticTarget::D)),
            0x03 => Some(Instruction::RLC(ArithmeticTarget::E)),
            0x04 => Some(Instruction::RLC(ArithmeticTarget::H)),
            0x05 => Some(Instruction::RLC(ArithmeticTarget::L)),
            0x06 => Some(Instruction::RLC(ArithmeticTarget::HLI)),
            0x07 => Some(Instruction::RLC(ArithmeticTarget::A)),

            // Rotate Right Carry Instructions
            0x08 => Some(Instruction::RRC(ArithmeticTarget::B)),
            0x09 => Some(Instruction::RRC(ArithmeticTarget::C)),
            0x0A => Some(Instruction::RRC(ArithmeticTarget::D)),
            0x0B => Some(Instruction::RRC(ArithmeticTarget::E)),
            0x0C => Some(Instruction::RRC(ArithmeticTarget::H)),
            0x0D => Some(Instruction::RRC(ArithmeticTarget::L)),
            0x0E => Some(Instruction::RRC(ArithmeticTarget::HLI)),
            0x0F => Some(Instruction::RRC(ArithmeticTarget::A)),

            //Rotate Left Instructions
            0x10 => Some(Instruction::RL(ArithmeticTarget::B)),
            0x11 => Some(Instruction::RL(ArithmeticTarget::C)),
            0x12 => Some(Instruction::RL(ArithmeticTarget::D)),
            0x13 => Some(Instruction::RL(ArithmeticTarget::E)),
            0x14 => Some(Instruction::RL(ArithmeticTarget::H)),
            0x15 => Some(Instruction::RL(ArithmeticTarget::L)),
            0x16 => Some(Instruction::RL(ArithmeticTarget::HLI)),
            0x17 => Some(Instruction::RL(ArithmeticTarget::A)),

            //Rotate Right Instructions
            0x18 => Some(Instruction::RR(ArithmeticTarget::B)),
            0x19 => Some(Instruction::RR(ArithmeticTarget::C)),
            0x1A => Some(Instruction::RR(ArithmeticTarget::D)),
            0x1B => Some(Instruction::RR(ArithmeticTarget::E)),
            0x1C => Some(Instruction::RR(ArithmeticTarget::H)),
            0x1D => Some(Instruction::RR(ArithmeticTarget::L)),
            0x1E => Some(Instruction::RR(ArithmeticTarget::HLI)),
            0x1F => Some(Instruction::RR(ArithmeticTarget::A)),

            // Shift Left Arithmetic Instructions
            0x20 => Some(Instruction::SLA(ArithmeticTarget::B)),
            0x21 => Some(Instruction::SLA(ArithmeticTarget::C)),
            0x22 => Some(Instruction::SLA(ArithmeticTarget::D)),
            0x23 => Some(Instruction::SLA(ArithmeticTarget::E)),
            0x24 => Some(Instruction::SLA(ArithmeticTarget::H)),
            0x25 => Some(Instruction::SLA(ArithmeticTarget::L)),
            0x26 => Some(Instruction::SLA(ArithmeticTarget::HLI)),
            0x27 => Some(Instruction::SLA(ArithmeticTarget::A)),

            // Shift Right Arithmetic Instructions
            0x28 => Some(Instruction::SRA(ArithmeticTarget::B)),
            0x29 => Some(Instruction::SRA(ArithmeticTarget::C)),
            0x2A => Some(Instruction::SRA(ArithmeticTarget::D)),
            0x2B => Some(Instruction::SRA(ArithmeticTarget::E)),
            0x2C => Some(Instruction::SRA(ArithmeticTarget::H)),
            0x2D => Some(Instruction::SRA(ArithmeticTarget::L)),
            0x2E => Some(Instruction::SRA(ArithmeticTarget::HLI)),
            0x2F => Some(Instruction::SRA(ArithmeticTarget::A)),

            // Swap Instructions
            0x30 => Some(Instruction::SWAP(ArithmeticTarget::B)),
            0x31 => Some(Instruction::SWAP(ArithmeticTarget::C)),
            0x32 => Some(Instruction::SWAP(ArithmeticTarget::D)),
            0x33 => Some(Instruction::SWAP(ArithmeticTarget::E)),
            0x34 => Some(Instruction::SWAP(ArithmeticTarget::H)),
            0x35 => Some(Instruction::SWAP(ArithmeticTarget::L)),
            0x36 => Some(Instruction::SWAP(ArithmeticTarget::HLI)),
            0x37 => Some(Instruction::SWAP(ArithmeticTarget::A)),
            // Shift Right Logical Instructions
            0x38 => Some(Instruction::SRL(ArithmeticTarget::B)),
            0x39 => Some(Instruction::SRL(ArithmeticTarget::C)),
            0x3A => Some(Instruction::SRL(ArithmeticTarget::D)),
            0x3B => Some(Instruction::SRL(ArithmeticTarget::E)),
            0x3C => Some(Instruction::SRL(ArithmeticTarget::H)),
            0x3D => Some(Instruction::SRL(ArithmeticTarget::L)),
            0x3E => Some(Instruction::SRL(ArithmeticTarget::HLI)),
            0x3F => Some(Instruction::SRL(ArithmeticTarget::A)),

            // Bit instructions
            0x40 => Some(Instruction::BIT(0x00, ArithmeticTarget::B)),
            0x41 => Some(Instruction::BIT(0x00, ArithmeticTarget::C)),
            0x42 => Some(Instruction::BIT(0x00, ArithmeticTarget::D)),
            0x43 => Some(Instruction::BIT(0x00, ArithmeticTarget::E)),
            0x44 => Some(Instruction::BIT(0x00, ArithmeticTarget::H)),
            0x45 => Some(Instruction::BIT(0x00, ArithmeticTarget::L)),
            0x46 => Some(Instruction::BIT(0x00, ArithmeticTarget::HLI)),
            0x47 => Some(Instruction::BIT(0x00, ArithmeticTarget::A)),
            0x48 => Some(Instruction::BIT(0x01, ArithmeticTarget::B)),
            0x49 => Some(Instruction::BIT(0x01, ArithmeticTarget::C)),
            0x4A => Some(Instruction::BIT(0x01, ArithmeticTarget::D)),
            0x4B => Some(Instruction::BIT(0x01, ArithmeticTarget::E)),
            0x4C => Some(Instruction::BIT(0x01, ArithmeticTarget::H)),
            0x4D => Some(Instruction::BIT(0x01, ArithmeticTarget::L)),
            0x4E => Some(Instruction::BIT(0x01, ArithmeticTarget::HLI)),
            0x4F => Some(Instruction::BIT(0x01, ArithmeticTarget::A)),
            0x50 => Some(Instruction::BIT(0x02, ArithmeticTarget::B)),
            0x51 => Some(Instruction::BIT(0x02, ArithmeticTarget::C)),
            0x52 => Some(Instruction::BIT(0x02, ArithmeticTarget::D)),
            0x53 => Some(Instruction::BIT(0x02, ArithmeticTarget::E)),
            0x54 => Some(Instruction::BIT(0x02, ArithmeticTarget::H)),
            0x55 => Some(Instruction::BIT(0x02, ArithmeticTarget::L)),
            0x56 => Some(Instruction::BIT(0x02, ArithmeticTarget::HLI)),
            0x57 => Some(Instruction::BIT(0x02, ArithmeticTarget::A)),
            0x58 => Some(Instruction::BIT(0x03, ArithmeticTarget::B)),
            0x59 => Some(Instruction::BIT(0x03, ArithmeticTarget::C)),
            0x5A => Some(Instruction::BIT(0x03, ArithmeticTarget::D)),
            0x5B => Some(Instruction::BIT(0x03, ArithmeticTarget::E)),
            0x5C => Some(Instruction::BIT(0x03, ArithmeticTarget::H)),
            0x5D => Some(Instruction::BIT(0x03, ArithmeticTarget::L)),
            0x5E => Some(Instruction::BIT(0x03, ArithmeticTarget::HLI)),
            0x5F => Some(Instruction::BIT(0x03, ArithmeticTarget::A)),
            0x60 => Some(Instruction::BIT(0x04, ArithmeticTarget::B)),
            0x61 => Some(Instruction::BIT(0x04, ArithmeticTarget::C)),
            0x62 => Some(Instruction::BIT(0x04, ArithmeticTarget::D)),
            0x63 => Some(Instruction::BIT(0x04, ArithmeticTarget::E)),
            0x64 => Some(Instruction::BIT(0x04, ArithmeticTarget::H)),
            0x65 => Some(Instruction::BIT(0x04, ArithmeticTarget::L)),
            0x66 => Some(Instruction::BIT(0x04, ArithmeticTarget::HLI)),
            0x67 => Some(Instruction::BIT(0x04, ArithmeticTarget::A)),
            0x68 => Some(Instruction::BIT(0x05, ArithmeticTarget::B)),
            0x69 => Some(Instruction::BIT(0x05, ArithmeticTarget::C)),
            0x6A => Some(Instruction::BIT(0x05, ArithmeticTarget::D)),
            0x6B => Some(Instruction::BIT(0x05, ArithmeticTarget::E)),
            0x6C => Some(Instruction::BIT(0x05, ArithmeticTarget::H)),
            0x6D => Some(Instruction::BIT(0x05, ArithmeticTarget::L)),
            0x6E => Some(Instruction::BIT(0x05, ArithmeticTarget::HLI)),
            0x6F => Some(Instruction::BIT(0x05, ArithmeticTarget::A)),
            0x70 => Some(Instruction::BIT(0x06, ArithmeticTarget::B)),
            0x71 => Some(Instruction::BIT(0x06, ArithmeticTarget::C)),
            0x72 => Some(Instruction::BIT(0x06, ArithmeticTarget::D)),
            0x73 => Some(Instruction::BIT(0x06, ArithmeticTarget::E)),
            0x74 => Some(Instruction::BIT(0x06, ArithmeticTarget::H)),
            0x75 => Some(Instruction::BIT(0x06, ArithmeticTarget::L)),
            0x76 => Some(Instruction::BIT(0x06, ArithmeticTarget::HLI)),
            0x77 => Some(Instruction::BIT(0x06, ArithmeticTarget::A)),
            0x78 => Some(Instruction::BIT(0x07, ArithmeticTarget::B)),
            0x79 => Some(Instruction::BIT(0x07, ArithmeticTarget::C)),
            0x7A => Some(Instruction::BIT(0x07, ArithmeticTarget::D)),
            0x7B => Some(Instruction::BIT(0x07, ArithmeticTarget::E)),
            0x7C => Some(Instruction::BIT(0x07, ArithmeticTarget::H)),
            0x7D => Some(Instruction::BIT(0x07, ArithmeticTarget::L)),
            0x7E => Some(Instruction::BIT(0x07, ArithmeticTarget::HLI)),
            0x7F => Some(Instruction::BIT(0x07, ArithmeticTarget::A)),

            // Reset instructions
            0x80 => Some(Instruction::RES(0x00, ArithmeticTarget::B)),
            0x81 => Some(Instruction::RES(0x00, ArithmeticTarget::C)),
            0x82 => Some(Instruction::RES(0x00, ArithmeticTarget::D)),
            0x83 => Some(Instruction::RES(0x00, ArithmeticTarget::E)),
            0x84 => Some(Instruction::RES(0x00, ArithmeticTarget::H)),
            0x85 => Some(Instruction::RES(0x00, ArithmeticTarget::L)),
            0x86 => Some(Instruction::RES(0x00, ArithmeticTarget::HLI)),
            0x87 => Some(Instruction::RES(0x00, ArithmeticTarget::A)),
            0x88 => Some(Instruction::RES(0x01, ArithmeticTarget::B)),
            0x89 => Some(Instruction::RES(0x01, ArithmeticTarget::C)),
            0x8A => Some(Instruction::RES(0x01, ArithmeticTarget::D)),
            0x8B => Some(Instruction::RES(0x01, ArithmeticTarget::E)),
            0x8C => Some(Instruction::RES(0x01, ArithmeticTarget::H)),
            0x8D => Some(Instruction::RES(0x01, ArithmeticTarget::L)),
            0x8E => Some(Instruction::RES(0x01, ArithmeticTarget::HLI)),
            0x8F => Some(Instruction::RES(0x01, ArithmeticTarget::A)),
            0x90 => Some(Instruction::RES(0x02, ArithmeticTarget::B)),
            0x91 => Some(Instruction::RES(0x02, ArithmeticTarget::C)),
            0x92 => Some(Instruction::RES(0x02, ArithmeticTarget::D)),
            0x93 => Some(Instruction::RES(0x02, ArithmeticTarget::E)),
            0x94 => Some(Instruction::RES(0x02, ArithmeticTarget::H)),
            0x95 => Some(Instruction::RES(0x02, ArithmeticTarget::L)),
            0x96 => Some(Instruction::RES(0x02, ArithmeticTarget::HLI)),
            0x97 => Some(Instruction::RES(0x02, ArithmeticTarget::A)),
            0x98 => Some(Instruction::RES(0x03, ArithmeticTarget::B)),
            0x99 => Some(Instruction::RES(0x03, ArithmeticTarget::C)),
            0x9A => Some(Instruction::RES(0x03, ArithmeticTarget::D)),
            0x9B => Some(Instruction::RES(0x03, ArithmeticTarget::E)),
            0x9C => Some(Instruction::RES(0x03, ArithmeticTarget::H)),
            0x9D => Some(Instruction::RES(0x03, ArithmeticTarget::L)),
            0x9E => Some(Instruction::RES(0x03, ArithmeticTarget::HLI)),
            0x9F => Some(Instruction::RES(0x03, ArithmeticTarget::A)),
            0xA0 => Some(Instruction::RES(0x04, ArithmeticTarget::B)),
            0xA1 => Some(Instruction::RES(0x04, ArithmeticTarget::C)),
            0xA2 => Some(Instruction::RES(0x04, ArithmeticTarget::D)),
            0xA3 => Some(Instruction::RES(0x04, ArithmeticTarget::E)),
            0xA4 => Some(Instruction::RES(0x04, ArithmeticTarget::H)),
            0xA5 => Some(Instruction::RES(0x04, ArithmeticTarget::L)),
            0xA6 => Some(Instruction::RES(0x04, ArithmeticTarget::HLI)),
            0xA7 => Some(Instruction::RES(0x04, ArithmeticTarget::A)),
            0xA8 => Some(Instruction::RES(0x05, ArithmeticTarget::B)),
            0xA9 => Some(Instruction::RES(0x05, ArithmeticTarget::C)),
            0xAA => Some(Instruction::RES(0x05, ArithmeticTarget::D)),
            0xAB => Some(Instruction::RES(0x05, ArithmeticTarget::E)),
            0xAC => Some(Instruction::RES(0x05, ArithmeticTarget::H)),
            0xAD => Some(Instruction::RES(0x05, ArithmeticTarget::L)),
            0xAE => Some(Instruction::RES(0x05, ArithmeticTarget::HLI)),
            0xAF => Some(Instruction::RES(0x05, ArithmeticTarget::A)),
            0xB0 => Some(Instruction::RES(0x06, ArithmeticTarget::B)),
            0xB1 => Some(Instruction::RES(0x06, ArithmeticTarget::C)),
            0xB2 => Some(Instruction::RES(0x06, ArithmeticTarget::D)),
            0xB3 => Some(Instruction::RES(0x06, ArithmeticTarget::E)),
            0xB4 => Some(Instruction::RES(0x06, ArithmeticTarget::H)),
            0xB5 => Some(Instruction::RES(0x06, ArithmeticTarget::L)),
            0xB6 => Some(Instruction::RES(0x06, ArithmeticTarget::HLI)),
            0xB7 => Some(Instruction::RES(0x06, ArithmeticTarget::A)),
            0xB8 => Some(Instruction::RES(0x07, ArithmeticTarget::B)),
            0xB9 => Some(Instruction::RES(0x07, ArithmeticTarget::C)),
            0xBA => Some(Instruction::RES(0x07, ArithmeticTarget::D)),
            0xBB => Some(Instruction::RES(0x07, ArithmeticTarget::E)),
            0xBC => Some(Instruction::RES(0x07, ArithmeticTarget::H)),
            0xBD => Some(Instruction::RES(0x07, ArithmeticTarget::L)),
            0xBE => Some(Instruction::RES(0x07, ArithmeticTarget::HLI)),
            0xBF => Some(Instruction::RES(0x07, ArithmeticTarget::A)),

            // Set instructions
            0xC0 => Some(Instruction::SET(0x00, ArithmeticTarget::B)),
            0xC1 => Some(Instruction::SET(0x00, ArithmeticTarget::C)),
            0xC2 => Some(Instruction::SET(0x00, ArithmeticTarget::D)),
            0xC3 => Some(Instruction::SET(0x00, ArithmeticTarget::E)),
            0xC4 => Some(Instruction::SET(0x00, ArithmeticTarget::H)),
            0xC5 => Some(Instruction::SET(0x00, ArithmeticTarget::L)),
            0xC6 => Some(Instruction::SET(0x00, ArithmeticTarget::HLI)),
            0xC7 => Some(Instruction::SET(0x00, ArithmeticTarget::A)),
            0xC8 => Some(Instruction::SET(0x01, ArithmeticTarget::B)),
            0xC9 => Some(Instruction::SET(0x01, ArithmeticTarget::C)),
            0xCA => Some(Instruction::SET(0x01, ArithmeticTarget::D)),
            0xCB => Some(Instruction::SET(0x01, ArithmeticTarget::E)),
            0xCC => Some(Instruction::SET(0x01, ArithmeticTarget::H)),
            0xCD => Some(Instruction::SET(0x01, ArithmeticTarget::L)),
            0xCE => Some(Instruction::SET(0x01, ArithmeticTarget::HLI)),
            0xCF => Some(Instruction::SET(0x01, ArithmeticTarget::A)),
            0xD0 => Some(Instruction::SET(0x02, ArithmeticTarget::B)),
            0xD1 => Some(Instruction::SET(0x02, ArithmeticTarget::C)),
            0xD2 => Some(Instruction::SET(0x02, ArithmeticTarget::D)),
            0xD3 => Some(Instruction::SET(0x02, ArithmeticTarget::E)),
            0xD4 => Some(Instruction::SET(0x02, ArithmeticTarget::H)),
            0xD5 => Some(Instruction::SET(0x02, ArithmeticTarget::L)),
            0xD6 => Some(Instruction::SET(0x02, ArithmeticTarget::HLI)),
            0xD7 => Some(Instruction::SET(0x02, ArithmeticTarget::A)),
            0xD8 => Some(Instruction::SET(0x03, ArithmeticTarget::B)),
            0xD9 => Some(Instruction::SET(0x03, ArithmeticTarget::C)),
            0xDA => Some(Instruction::SET(0x03, ArithmeticTarget::D)),
            0xDB => Some(Instruction::SET(0x03, ArithmeticTarget::E)),
            0xDC => Some(Instruction::SET(0x03, ArithmeticTarget::H)),
            0xDD => Some(Instruction::SET(0x03, ArithmeticTarget::L)),
            0xDE => Some(Instruction::SET(0x03, ArithmeticTarget::HLI)),
            0xDF => Some(Instruction::SET(0x03, ArithmeticTarget::A)),
            0xE0 => Some(Instruction::SET(0x04, ArithmeticTarget::B)),
            0xE1 => Some(Instruction::SET(0x04, ArithmeticTarget::C)),
            0xE2 => Some(Instruction::SET(0x04, ArithmeticTarget::D)),
            0xE3 => Some(Instruction::SET(0x04, ArithmeticTarget::E)),
            0xE4 => Some(Instruction::SET(0x04, ArithmeticTarget::H)),
            0xE5 => Some(Instruction::SET(0x04, ArithmeticTarget::L)),
            0xE6 => Some(Instruction::SET(0x04, ArithmeticTarget::HLI)),
            0xE7 => Some(Instruction::SET(0x04, ArithmeticTarget::A)),
            0xE8 => Some(Instruction::SET(0x05, ArithmeticTarget::B)),
            0xE9 => Some(Instruction::SET(0x05, ArithmeticTarget::C)),
            0xEA => Some(Instruction::SET(0x05, ArithmeticTarget::D)),
            0xEB => Some(Instruction::SET(0x05, ArithmeticTarget::E)),
            0xEC => Some(Instruction::SET(0x05, ArithmeticTarget::H)),
            0xED => Some(Instruction::SET(0x05, ArithmeticTarget::L)),
            0xEE => Some(Instruction::SET(0x05, ArithmeticTarget::HLI)),
            0xEF => Some(Instruction::SET(0x05, ArithmeticTarget::A)),
            0xF0 => Some(Instruction::SET(0x06, ArithmeticTarget::B)),
            0xF1 => Some(Instruction::SET(0x06, ArithmeticTarget::C)),
            0xF2 => Some(Instruction::SET(0x06, ArithmeticTarget::D)),
            0xF3 => Some(Instruction::SET(0x06, ArithmeticTarget::E)),
            0xF4 => Some(Instruction::SET(0x06, ArithmeticTarget::H)),
            0xF5 => Some(Instruction::SET(0x06, ArithmeticTarget::L)),
            0xF6 => Some(Instruction::SET(0x06, ArithmeticTarget::HLI)),
            0xF7 => Some(Instruction::SET(0x06, ArithmeticTarget::A)),
            0xF8 => Some(Instruction::SET(0x07, ArithmeticTarget::B)),
            0xF9 => Some(Instruction::SET(0x07, ArithmeticTarget::C)),
            0xFA => Some(Instruction::SET(0x07, ArithmeticTarget::D)),
            0xFB => Some(Instruction::SET(0x07, ArithmeticTarget::E)),
            0xFC => Some(Instruction::SET(0x07, ArithmeticTarget::H)),
            0xFD => Some(Instruction::SET(0x07, ArithmeticTarget::L)),
            0xFE => Some(Instruction::SET(0x07, ArithmeticTarget::HLI)),
            0xFF => Some(Instruction::SET(0x07, ArithmeticTarget::A)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // Add instructions
            0x09 => Some(Instruction::ADDL(ArithmeticTargetLong::BC)),
            0x19 => Some(Instruction::ADDL(ArithmeticTargetLong::DE)),
            0x29 => Some(Instruction::ADDL(ArithmeticTargetLong::HL)),
            0x39 => Some(Instruction::ADDL(ArithmeticTargetLong::SP)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            // 0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            // 0xE8 => Some(Instruction::ADD(ArithmeticTarget::S8)), // S8 + SP
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HLI)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            // 0xCE => Some(Instruction::ADC(ArithmeticTarget::D8)),

            // Sub instructions
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HLI)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            // 0xD6 => Some(Instruction::SUB(ArithmeticTarget::D8)),\
            0x98 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SUB(ArithmeticTarget::HLI)),
            0x9F => Some(Instruction::SUB(ArithmeticTarget::A)),
            // 0xDE => Some(Instruction::SUB(ArithmeticTarget::D8)),

            // Compare Instructions
            0xB8 => Some(Instruction::CMP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CMP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CMP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CMP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CMP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CMP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CMP(ArithmeticTarget::HLI)),
            0xBF => Some(Instruction::CMP(ArithmeticTarget::A)),
            // 0xFE => Some(Instruction::CMP(ArithmeticTarget::D8)),

            // Increment Instructions
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),

            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x34 => Some(Instruction::INC(IncDecTarget::HLI)),

            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),

            // Decrement Instructions
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HLI)),

            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            0x3B => Some(Instruction::DEC(IncDecTarget::SP)),

            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),

            // Bitwise AND
            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HLI)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            // 0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),

            // Bitwise OR
            0xB0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::AND(ArithmeticTarget::HLI)),
            0xB7 => Some(Instruction::AND(ArithmeticTarget::A)),
            // 0xF6 => Some(Instruction::AND(ArithmeticTarget::D8)),

            // Bitwise XOR
            0xA8 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xAA => Some(Instruction::AND(ArithmeticTarget::D)),
            0xAB => Some(Instruction::AND(ArithmeticTarget::E)),
            0xAC => Some(Instruction::AND(ArithmeticTarget::H)),
            0xAD => Some(Instruction::AND(ArithmeticTarget::L)),
            0xAE => Some(Instruction::AND(ArithmeticTarget::HLI)),
            0xAF => Some(Instruction::AND(ArithmeticTarget::A)),
            // 0xEE => Some(Instruction::AND(ArithmeticTarget::D8)),

            // Other instructions
            0x3F => Some(Instruction::CCF),
            0x37 => Some(Instruction::SCF),
            0x27 => Some(Instruction::DAA),
            0x2F => Some(Instruction::CPL),

            // Rotate Instructions
            0x07 => Some(Instruction::RLCA),
            0x0F => Some(Instruction::RRCA),
            0x17 => Some(Instruction::RLA),
            0x1F => Some(Instruction::RRA),

            _ => None,
        }
    }
}

// Enums for arithmetic instructions
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum IncDecTarget {
    BC,
    DE,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    SP,
    HLI,
}

pub enum ArithmeticTargetLong {
    BC,
    HL,
    DE,
    SP,
}

// Enums for jump instructions
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

// Enums for load instructions
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLT,
}
