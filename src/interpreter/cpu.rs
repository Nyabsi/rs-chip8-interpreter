// This file manages the CPU of the interpreter.

use std::ops::Deref;

use super::memory::Memory;

enum Instructions {
    INSTRUCTION_UNKNOWN = 0,
    INSTRUCTION_CLEAR_SCREEN = 0x00E0,
}
pub struct CPU {
    memory: Memory,
    pc_reg: u16,
    i_reg: u16,
    sp_reg: u16,
    stack: [u16; 16],
    v_regs: [u8; 16],    // 15 variable registers, 1 flag register.
    current_opcode: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory: Memory::new(),
            pc_reg: 0x0,
            i_reg: 0x0,
            sp_reg: 0x0,
            stack: [0; 16],
            v_regs: [0; 16],
            current_opcode: 0x0,
        }
    }
    pub fn initialize(&mut self) {
        self.pc_reg = 0x200; // point our Program Counter at the beginning of our application in the memory.
    }
    // Let's only expose "execute" publicly, we'll handle fetching and decoding privately.
    pub fn execute(&mut self) {
        let opcode = self.fetch();
        self.current_opcode = opcode;
        let instruction = self.decode(opcode);
        match instruction {
            _ => {
                panic!("Unimplemented Instruction, 0x{} called!", opcode);
            }
        }
    }
    pub fn get_memory(&mut self) -> &mut Memory {
        return &mut self.memory; // return reference to the memory management.
    }
    fn fetch(&mut self) -> u16 {
        let lP = self.memory.get_from_index(self.pc_reg.into()).clone();
        let hP = self.memory.get_from_index((self.pc_reg + 1).into()).clone();
        return (u16::from(lP) << 8 | u16::from(hP));
    }
    fn decode(&mut self, opcode: u16) -> Instructions {
        match opcode {
            _ if opcode & 0xf000 != 0 => {
                match opcode {
                    0x0000 => {
                        match opcode {
                            _ if opcode & 0x00ff != 0 => {
                                match opcode {
                                    0x00E0 => {
                                        return Instructions::INSTRUCTION_CLEAR_SCREEN;
                                    },
                                    _ => {
                                        panic!("Unknown Instruction (0x0FF family), opcode: 0x{}", opcode);
                                    }
                                }
                            },
                            _ => panic!("Unknown Instruction Family, opcode: 0x{}", opcode), // We should never reach here.
                        }
                    },
                    _ => {
                        panic!("Unknown Instruction, opcode: 0x{}", opcode);
                    }
                }
            },
            _ => {
                panic!("Unknown Instruction (0xF000 family), opcode: 0x{}", opcode);
            },
        }
    }
}