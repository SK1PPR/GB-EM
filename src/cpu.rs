use crate::instructions::{ArithmeticTarget, IncDecTarget, Instruction, JumpType};
use crate::memory::MemoryBus;
use crate::registers::Registers;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}:{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unknown instruction found for: {}", description);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::NOP => self.pc.wrapping_add(1),

            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpType::NotZero => !self.registers.f.zero,
                    JumpType::Zero => self.registers.f.zero,
                    JumpType::NotCarry => !self.registers.f.carry,
                    JumpType::Carry => self.registers.f.carry,
                    JumpType::Always => true,
                };
                self.jump(jump_condition)
            }
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        /* Add more targets */
                        self.pc
                    }
                }
            }
            _ => {
                /* Support more instructions */
                self.pc
            }
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

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }
}
