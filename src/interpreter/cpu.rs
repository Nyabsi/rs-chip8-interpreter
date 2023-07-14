// This file manages the CPU of the interpreter.

use super::memory::Memory;

enum Instructions {
    InstructionUnknown = 0,
    InstructionClearScreen = 0x1,
    InstructionJump = 0x2,
    InstructionCallSubroutine = 0x3,
    InstructionSetRegister = 0x4,
}
pub struct CPU {
    memory: Memory,
    pc_reg: u16,
    i_reg: u16,
    sp_reg: u16,
    stack: [u16; 16],
    v_regs: [u8; 16],    // 15 variable registers, 1 flag register.
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
        }
    }
    pub fn initialize(&mut self) {
        self.pc_reg = 0x200; // point our Program Counter at the beginning of our application in the memory.
    }
    // Let's only expose "execute" publicly, we'll handle fetching and decoding privately.
    pub fn execute(&mut self) {
        let opcode = self.fetch();
        let instruction = self.decode(opcode);
        match instruction {
            Instructions::InstructionJump => {
                let nnn: u16 = opcode & 0x0fff;
                self.pc_reg = nnn;
            }
            Instructions::InstructionCallSubroutine => {
                let nnn: u16 = opcode & 0x0fff;
                self.stack[self.sp_reg as usize] = nnn;
                self.sp_reg += 1;
                self.pc_reg = nnn;
            }
            Instructions::InstructionSetRegister => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let nn = (opcode & 0x00ff) as u8;
                self.v_regs[vx as usize] = nn;
                self.pc_reg += 2;
            }
            _ => {
                panic!("Unimplemented Instruction, 0x{:X} called!", opcode);
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
        match opcode & 0xf000 {
            0x0000 => {
                match opcode & 0x00ff {
                    0x00E0 => {
                        return Instructions::InstructionClearScreen
                    }
                    _ => {
                        panic!("Unknown Instruction (0x00FF family), opcode: 0x{:X}", opcode);
                    }
                }
            },
            0x1000 => {
                return Instructions::InstructionJump
            },
            0x2000 => {
                return Instructions::InstructionCallSubroutine
            },
            0x6000 => {
                return Instructions::InstructionSetRegister
            }
            _ => {
                panic!("Unknown Instruction, opcode: 0x{:X}", opcode);
            }
        }
    }
}