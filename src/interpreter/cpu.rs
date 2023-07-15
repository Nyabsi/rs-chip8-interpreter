// This file manages the CPU of the interpreter.

use super::memory::Memory;

enum Instructions {
    Instruction00e0 = 0x1,  // Clear Display
    Instruction1nnn = 0x2,  // Jump
    Instruction2nnn = 0x3,  // Call Subroutine
    Instruction6xnn = 0x4,  // Set
    Instructionannn = 0x5,  // Set Index
    Instructiondxyn = 0x6,  // Draw
    Instruction7xnn = 0x7,  // Add
    Instructionfx33 = 0x8,  // Binary-coded decimal conversion
    Instruction3xnn = 0x9,  // Skip
    Instruction4xnn = 0x10, // Skip
    Instruction5xy0 = 0x11, // Skip
    Instruction9xy0 = 0x12, // Skip
    Instruction00ee = 0x13, // Return Subroutine
    Instruction8xy0 = 0x14, // Copy
    Instruction8xy1 = 0x15, // Binary OR
    Instruction8xy2 = 0x16, // Binary AND
    Instruction8xy3 = 0x17, // Logical XOR
    Instruction8xy4 = 0x18, // Sum
    Instruction8xy5 = 0x19, // Substract
    Instruction8xy7 = 0x20, // Substract (reverse)
}

pub struct CPU {
    buffer: [[bool; 64]; 32],
    pc: u16,
    i: u16,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16], // 15 variable registers, 1 flag register.
    flags: u16, // my specific flags which indicate what the processor is doing.
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            buffer: [[false; 64]; 32],
            pc: 0x200,
            i: 0x0,
            sp: 0x0,
            stack: [0; 16],
            v: [0; 16],
            flags: 0x0,
        }
    }

    // Let's only expose "execute" publicly, we'll handle fetching and decoding privately.
    pub fn execute(&mut self, memory: &mut Memory) {
        let opcode = self.fetch(memory);
        let instruction = self.decode(opcode);
        match instruction {
            Instructions::Instruction00e0 => {
                for y in 0..32 {
                    for x in 0..64 {
                        self.buffer[y][x] = false;
                    }
                }
                self.pc += 2;
                self.flags |= 0xA;
            },
            Instructions::Instruction1nnn => {
                let nnn: u16 = opcode & 0x0fff;
                self.pc = nnn;
            },
            Instructions::Instruction2nnn => {
                let nnn: u16 = opcode & 0x0fff;
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            Instructions::Instruction6xnn => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let nn = (opcode & 0x00ff) as u8;
                self.v[vx as usize] = nn;
                self.pc += 2;
            },
            Instructions::Instructionannn => {
                let nnn: u16 = opcode & 0x0fff;
                self.i = nnn;
                self.pc += 2;
            },
            Instructions::Instructiondxyn => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                let n = ((opcode & 0x000f)) as usize;
                let init_x = self.v[vx as usize] as usize;
                let init_y = self.v[vy as usize] as usize;
                
                self.v[0xF] = 0;
                for i in 0..n {
                    let pixel: u16 = memory.get_from_index(self.i as usize + i as usize) as u16;
                    for j in 0..8 {
                        let x = (init_x + j) % 64;
                        let y = (init_y + i) % 32;
                        if pixel & 0x80 >> j != 0 {
                            self.v[0xF] |= self.buffer[y % 32][x % 64] as u8;
                            self.buffer[y][x] ^= true;
                        }
                    }
                }

                self.pc += 2;
                self.flags |= 0xA;
            },
            Instructions::Instruction7xnn => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let nn = (opcode & 0x00ff) as u8;
                // prevent overflow
                self.v[vx as usize] = self.v[vx as usize].wrapping_add(nn) & 0xFF;
                self.pc += 2;
            },
            Instructions::Instructionfx33 => {
                let vx = (opcode & 0x0f00 >> 8) as u8;
                memory.set_from_index(self.i as usize, self.v[vx as usize] / 100);
                memory.set_from_index((self.i + 1) as usize, (self.v[vx as usize] / 10) % 10);
                memory.set_from_index((self.i  + 2) as usize, (self.v[vx as usize] % 100) % 10);
                self.pc += 2;
            },
            Instructions::Instruction3xnn => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let nn = (opcode & 0x00ff) as u8;
                if self.v[vx as usize] == nn { self.pc += 4; } else { self.pc += 2; }
            }
            Instructions::Instruction4xnn => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let nn = (opcode & 0x00ff) as u8;
                if self.v[vx as usize] != nn { self.pc += 4; } else { self.pc += 2; }
            },
            Instructions::Instruction5xy0 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                if self.v[vx as usize] == self.v[vy as usize] { self.pc += 4; } else { self.pc += 2; }
            },
            Instructions::Instruction9xy0 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                if self.v[vx as usize] != self.v[vy as usize] { self.pc += 4; } else { self.pc += 2; }
            },
            Instructions::Instruction00ee => {
                self.sp -= 1; // pop stack
                self.pc = self.stack[self.sp as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy2 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                self.v[vx as usize] = self.v[vx as usize] & self.v[vy as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy1 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                self.v[vx as usize] = self.v[vx as usize] | self.v[vy as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy3 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                self.v[vx as usize] = self.v[vx as usize] ^ self.v[vy as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy4 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                let sum = self.v[vx as usize] as u32 + self.v[vy as usize] as u32;
                self.v[0xF] = if sum > 0xFF { 1 } else { 0 };
                self.v[vx as usize] = self.v[vx as usize].wrapping_add(self.v[vy as usize]) & 0xFF;
                self.pc += 2;
            },
            Instructions::Instruction8xy0 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                self.v[vx as usize] = self.v[vy as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy5 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                self.v[0xF] = if self.v[vx as usize] > self.v[vy as usize] { 0 } else { 1 };
                self.v[vx as usize] = self.v[vx as usize].wrapping_sub(self.v[vy as usize]) & 0xFF;
                self.pc += 2;
            },
            Instructions::Instruction8xy7 => {
                let vx = ((opcode & 0x0f00) >> 8) as u8;
                let vy = ((opcode & 0x00f0) >> 4) as u8;
                self.v[0xF] = if self.v[vy as usize] > self.v[vx as usize] { 0 } else { 1 };
                self.v[vx as usize] = self.v[vy as usize].wrapping_sub(self.v[vx as usize]) & 0xFF;
                self.pc += 2;
            }
        }
    }

    fn fetch(&self, memory: &mut Memory) -> u16 {
        let part1 = memory.get_from_index(self.pc.into());
        let part2 = memory.get_from_index((self.pc + 1).into());
        return u16::from(part1) << 8 | u16::from(part2);
    }

    fn decode(&self, opcode: u16) -> Instructions {
        match opcode & 0xf000 {
            0x0000 => {
                match opcode & 0x00ff {
                    0x00E0 => {
                        Instructions::Instruction00e0
                    },
                    0x00EE => {
                        Instructions::Instruction00ee
                    },
                    _ => {
                        panic!("Unknown Instruction (0x0000 family), opcode: 0x{:X}", opcode);
                    }
                }
            },
            0x1000 => {
                Instructions::Instruction1nnn
            },
            0x2000 => {
                Instructions::Instruction2nnn
            },
            0x3000 => {
                Instructions::Instruction3xnn
            },
            0x4000 => {
                Instructions::Instruction4xnn
            },
            0x5000 => {
                Instructions::Instruction5xy0
            },
            0x6000 => {
                Instructions::Instruction6xnn
            },
            0x7000 => {
                Instructions::Instruction7xnn
            },
            0x8000 => {
                match opcode & 0x000f {
                    0x0000 => {
                        Instructions::Instruction8xy0
                    },
                    0x0001 => {
                        Instructions::Instruction8xy1
                    },
                    0x0002 => {
                        Instructions::Instruction8xy2
                    },
                    0x0003 => {
                        Instructions::Instruction8xy3
                    },
                    0x0004 => {
                        Instructions::Instruction8xy4
                    },
                    0x0005 => {
                        Instructions::Instruction8xy5
                    },
                    0x0007 => {
                        Instructions::Instruction8xy7
                    },
                    _ => {
                        panic!("Unknown Instruction (0x8000 family), opcode: 0x{:X}", opcode);
                    }
                }
            },
            0x9000 => {
                Instructions::Instruction9xy0
            },
            0xA000 => {
                Instructions::Instructionannn
            },
            0xD000 => {
                Instructions::Instructiondxyn
            },
            0xF000 => {
                match opcode & 0x00ff {
                    0x0033 => {
                        Instructions::Instructionfx33
                    }
                    _ => {
                        panic!("Unknown Instruction (0xF000 family), opcode: 0x{:X}", opcode);
                    }
                }
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

    pub fn get_buffer(&self) -> &[[bool; 64]; 32] {
        return &self.buffer;
    }
}