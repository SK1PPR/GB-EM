use crate::registers::Registers;
use crate::memory::MemoryBus;

enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
}

impl Instruction {
    fn from_byte(byte:u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
        }
    }
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
enum IncDecTarget {
    BC, DE,
}

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction found for: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => { /* Add more targets */ self.pc}
                }
            }
            _=> { /* Support more instructions */ self.pc}
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}