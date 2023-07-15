// This file manages the CPU of the interpreter.

use super::memory::Memory;

enum Instructions {
    InstructionUnknown = 0,
    InstructionClearScreen = 0x1,
    InstructionJump = 0x2,
    InstructionCallSubroutine = 0x3,
    InstructionSetRegister = 0x4,
    InstructionSetRegisterIndex = 0x5,
    InstructionRenderDisplay = 0x6,
    InstructionAddToRegister = 0x7,
}

pub struct CPU {
    screen_buffer: [[bool; 64]; 32],
    pc_reg: u16,
    i_reg: u16,
    sp_reg: u16,
    stack: [u16; 16],
    v_regs: [u8; 16],    // 15 variable registers, 1 flag register.
    flags: u16, // my specific flags which indicate what the processor is doing.
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            screen_buffer: [[false; 64]; 32],
            pc_reg: 0x0,
            i_reg: 0x0,
            sp_reg: 0x0,
            stack: [0; 16],
            v_regs: [0; 16],
            flags: 0x0,
        }
    }
    pub fn initialize(&mut self) {
        self.pc_reg = 0x200; // point our Program Counter at the beginning of our application in the memory.
    }
    // Let's only expose "execute" publicly, we'll handle fetching and decoding privately.
    pub fn execute(&mut self, memory: &mut Memory) {
        let opcode = self.fetch(memory);
        let instruction = self.decode(opcode);
        match instruction {
            Instructions::InstructionClearScreen => {
                for y in 0..32 {
                    for x in 0..64 {
                        self.screen_buffer[y][x] = false;
                    }
                }
                self.pc_reg += 2;
                self.flags |= 0xA;
            }
            Instructions::InstructionJump => {
                let nnn: u16 = opcode & 0x0fff;
                self.pc_reg = nnn;
            }
            Instructions::InstructionCallSubroutine => {
                let nnn: u16 = opcode & 0x0fff;
                self.stack[self.sp_reg as usize] = self.pc_reg;
                self.sp_reg += 1;
                self.pc_reg = nnn;
            }
            Instructions::InstructionSetRegister => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let nn = (opcode & 0x00ff) as u8;
                self.v_regs[vx as usize] = nn;
                self.pc_reg += 2;
            }
            Instructions::InstructionSetRegisterIndex => {
                let nnn: u16 = opcode & 0x0fff;
                self.i_reg = nnn;
                self.pc_reg += 2;
            }
            Instructions::InstructionRenderDisplay => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                let n = ((opcode & 0x000f)) as usize;
                let init_x = self.v_regs[vx as usize] as usize;
                let init_y = self.v_regs[vy as usize] as usize;
                
                self.v_regs[0xF] = 0;
                for i in 0..n {
                    let pixel: u16 = memory.get_from_index(self.i_reg as usize + i as usize) as u16;
                    for j in 0..8 {
                        let x = (init_x + j) % 64;
                        let y = (init_y + i) % 32;
                        if pixel & 0x80 >> j != 0 {
                            self.v_regs[0xF] |= self.screen_buffer[y % 32][x % 64] as u8;
                            self.screen_buffer[y][x] ^= true;
                        }
                    }
                }

                self.pc_reg += 2;
                self.flags |= 0xA;
            }
            Instructions::InstructionAddToRegister => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let nn = (opcode & 0x00ff) as u8;
                self.v_regs[vx as usize] += nn;
                self.pc_reg += 2;
            }
            _ => {
                panic!("Unimplemented Instruction, 0x{:X} called!", opcode);
            }
        }
    }
    fn fetch(&self, memory: &mut Memory) -> u16 {
        let lP = memory.get_from_index(self.pc_reg.into());
        let hP = memory.get_from_index((self.pc_reg + 1).into());
        return (u16::from(lP) << 8 | u16::from(hP));
    }
    fn decode(&self, opcode: u16) -> Instructions {
        match opcode & 0xf000 {
            0x0000 => {
                match opcode & 0x00ff {
                    0x00E0 => {
                        Instructions::InstructionClearScreen
                    }
                    _ => {
                        panic!("Unknown Instruction (0x00FF family), opcode: 0x{:X}", opcode);
                    }
                }
            },
            0x1000 => {
                Instructions::InstructionJump
            },
            0x2000 => {
                Instructions::InstructionCallSubroutine
            },
            0x6000 => {
                Instructions::InstructionSetRegister
            },
            0x7000 => {
                Instructions::InstructionAddToRegister
            }
            0xA000 => {
                Instructions::InstructionSetRegisterIndex
            },
            0xD000 => {
                Instructions::InstructionRenderDisplay
            }
            _ => {
                panic!("Unknown Instruction, opcode: 0x{:X}", opcode);
            }
        }
    }
    pub fn get_flags(&self) -> &u16 {
        return &self.flags;
    }
    pub fn remove_flag(&mut self, flag: u16) {
        self.flags &= !flag;
    }
    pub fn get_cpu_buffer(&self) -> &[[bool; 64]; 32] {
        return &self.screen_buffer;
    }
}