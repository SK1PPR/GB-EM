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

            // Prefixed Instructions
            Instruction::SLA(target) => {
                
                self.pc.wrapping_add(2)
            },

            Instruction::SRA(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.f.carry = self.registers.a & 0x01 == 1;
                        self.registers.a = (self.registers.a >> 1) | (self.registers.a & 0x80);
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.carry = self.registers.b & 0x01 == 1;
                        self.registers.b = (self.registers.b >> 1) | (self.registers.b & 0x80);
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.carry = self.registers.c & 0x01 == 1;
                        self.registers.c = (self.registers.c >> 1) | (self.registers.c & 0x80);
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.carry = self.registers.d & 0x01 == 1;
                        self.registers.d = (self.registers.d >> 1) | (self.registers.d & 0x80);
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.carry = self.registers.e & 0x01 == 1;
                        self.registers.e = (self.registers.e >> 1) | (self.registers.e & 0x80);
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.carry = self.registers.h & 0x01 == 1;
                        self.registers.h = (self.registers.h >> 1) | (self.registers.h & 0x80);
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.carry = self.registers.l & 0x01 == 1;
                        self.registers.l = (self.registers.l >> 1) | (self.registers.l & 0x80);
                    }
                    ArithmeticTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        self.registers.f.carry = value & 0x01 == 1;
                        self.bus
                            .set_byte(self.registers.get_hl(), (value >> 1) | (value & 0x80));
                    }
                }
                self.pc.wrapping_add(2)
            }

            Instruction::SWAP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = (self.registers.a << 4) | (self.registers.a >> 4)
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = (self.registers.b << 4) | (self.registers.b >> 4)
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = (self.registers.c << 4) | (self.registers.c >> 4)
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = (self.registers.d << 4) | (self.registers.d >> 4)
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = (self.registers.e << 4) | (self.registers.e >> 4)
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = (self.registers.h << 4) | (self.registers.h >> 4)
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = (self.registers.l << 4) | (self.registers.l >> 4)
                    }
                    ArithmeticTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        self.bus
                            .set_byte(self.registers.get_hl(), (value << 4) | (value >> 4));
                    }
                }
                self.pc.wrapping_add(2)
            }

            Instruction::SRL(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.f.carry = self.registers.a & 0x01 == 1;
                        self.registers.a = (self.registers.a >> 1) & 0x7F;
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.carry = self.registers.b & 0x01 == 1;
                        self.registers.b = (self.registers.b >> 1) & 0x7F;
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.carry = self.registers.c & 0x01 == 1;
                        self.registers.c = (self.registers.c >> 1) & 0x7F;
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.carry = self.registers.d & 0x01 == 1;
                        self.registers.d = (self.registers.d >> 1) & 0x7F;
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.carry = self.registers.e & 0x01 == 1;
                        self.registers.e = (self.registers.e >> 1) & 0x7F;
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.carry = self.registers.h & 0x01 == 1;
                        self.registers.h = (self.registers.h >> 1) & 0x7F;
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.carry = self.registers.l & 0x01 == 1;
                        self.registers.l = (self.registers.l >> 1) & 0x7F;
                    }
                    ArithmeticTarget::HLI => {
                        self.registers.f.carry =
                            self.bus.read_byte(self.registers.get_hl()) & 0x01 == 1;
                        self.bus.set_byte(
                            self.registers.get_hl(),
                            (self.bus.read_byte(self.registers.get_hl())) >> 1 & 0x7F,
                        )
                    }
                }
                self.pc.wrapping_add(2)
            }

            Instruction::BIT(bit, target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.f.zero = self.registers.a & 0x01 << bit == 0
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.zero = self.registers.b & 0x01 << bit == 0
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.zero = self.registers.c & 0x01 << bit == 0
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.zero = self.registers.d & 0x01 << bit == 0
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.zero = self.registers.e & 0x01 << bit == 0
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.zero = self.registers.h & 0x01 << bit == 0
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.zero = self.registers.l & 0x01 << bit == 0
                    }
                    ArithmeticTarget::HLI => {
                        self.registers.f.zero =
                            self.bus.read_byte(self.registers.get_hl()) & 0x01 << bit == 0
                    }
                }
                self.pc.wrapping_add(2)
            }

            Instruction::RES(bit, target) => {
                let value: u8 = 0xFF ^ (0x01 << bit);
                match target {
                    ArithmeticTarget::A => self.registers.b = self.registers.a & value,
                    ArithmeticTarget::B => self.registers.b = self.registers.b & value,
                    ArithmeticTarget::C => self.registers.b = self.registers.c & value,
                    ArithmeticTarget::D => self.registers.b = self.registers.d & value,
                    ArithmeticTarget::E => self.registers.b = self.registers.e & value,
                    ArithmeticTarget::H => self.registers.b = self.registers.h & value,
                    ArithmeticTarget::L => self.registers.b = self.registers.l & value,
                    ArithmeticTarget::HLI => self.bus.set_byte(
                        self.registers.get_hl(),
                        self.bus.read_byte(self.registers.get_hl()) & value,
                    ),
                }
                self.pc.wrapping_add(2)
            }

            Instruction::SET(bit, target) => {
                match target {
                    ArithmeticTarget::A => self.registers.b = self.registers.a | 0x01 << bit,
                    ArithmeticTarget::B => self.registers.b = self.registers.b | 0x01 << bit,
                    ArithmeticTarget::C => self.registers.b = self.registers.c | 0x01 << bit,
                    ArithmeticTarget::D => self.registers.b = self.registers.d | 0x01 << bit,
                    ArithmeticTarget::E => self.registers.b = self.registers.e | 0x01 << bit,
                    ArithmeticTarget::H => self.registers.b = self.registers.h | 0x01 << bit,
                    ArithmeticTarget::L => self.registers.b = self.registers.l | 0x01 << bit,
                    ArithmeticTarget::HLI => self.bus.set_byte(
                        self.registers.get_hl(),
                        self.bus.read_byte(self.registers.get_hl()) | 0x01 << bit,
                    ),
                }
                self.pc.wrapping_add(2)
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
