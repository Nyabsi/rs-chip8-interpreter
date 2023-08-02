// MIT License
// 
// Copyright (c) 2023 LumenTuoma
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// UTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use rand::Rng;

use super::display::Display;
use super::memory::Memory;
use super::keypad::Keypad;


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
    Instruction8xy6 = 0x20, // Shift (Right)
    Instruction8xy7 = 0x21, // Substract (reverse)
    Instruction8xye = 0x22, // Shift (Left)
    Instructionfx55 = 0x23, // Store Memory
    Instructionfx65 = 0x24, // Load Memory
    Instructionfx29 = 0x25, // Font character
    Instructioncxnn = 0x26, // Random
    Instructionbnnn = 0x27, // Jump with offset
    Instructionfx1e = 0x28, // Add to index
    Instructionfx0a = 0x29, // Get key
    Instructionfx07 = 0x30, // Timer (Delay) Fetch
    Instructionfx15 = 0x31, // Timer (Delay) Set
    Instructionfx18 = 0x32, // Timer (Sound) Set
    Instructionex9e = 0x33, // Skip if key
    Instructionexa1 = 0x34, // Skip if key
}

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct CPU {
    buffer: [[bool; WIDTH]; HEIGHT],
    pc: u16,
    i: u16,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            buffer: [[false; WIDTH]; HEIGHT],
            pc: 0x200,
            i: 0x0,
            sp: 0x0,
            stack: [0; 16],
            v: [0; 16],
            delay_timer: 0x0,
            sound_timer: 0x0
        }
        
    }

    // Let's only expose "execute" publicly, we'll handle fetching and decoding privately.
    pub fn execute(&mut self, display: &mut Display, memory: &mut Memory, keypad:&mut Keypad) {
        let opcode = self.fetch(memory);
        let instruction = self.decode(opcode);

        // XY
        let x = (opcode & 0x0f00) >> 8;
        let y = (opcode & 0x00f0) >> 4;

        match instruction {
            Instructions::Instruction00e0 => {
                for y in 0..32 {
                    for x in 0..64 {
                        self.buffer[y][x] = false;
                    }
                }
                self.pc += 2;
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
                let nn = opcode & 0x00ff;
                self.v[x as usize] = nn as u8;
                self.pc += 2;
            },
            Instructions::Instructionannn => {
                let nnn: u16 = opcode & 0x0fff;
                self.i = nnn;
                self.pc += 2;
            },
            Instructions::Instructiondxyn => {
                let n = (opcode & 0x000f) as usize;
                let init_x = self.v[x as usize] as usize;
                let init_y = self.v[y as usize] as usize;
                
                self.v[0xF] = 0;
                for i in 0..n {
                    let pixel: u16 = memory.get_from_index(self.i as usize + i as usize) as u16;
                    for j in 0..8 {
                        let x = (init_x + j) % WIDTH;
                        let y = (init_y + i) % HEIGHT;
                        if pixel & 0x80 >> j != 0 {
                            self.v[0xF] |= self.buffer[y % HEIGHT][x % WIDTH] as u8;
                            self.buffer[y][x] ^= true;
                        }
                    }
                }
                display.draw(&self.buffer);
                self.pc += 2;
            },
            Instructions::Instruction7xnn => {
                let nn = opcode & 0x00ff;
                self.v[x as usize] = self.v[x as usize].wrapping_add(nn as u8) & 0xFF;
                self.pc += 2;
            },
            Instructions::Instructionfx33 => {
                let v = self.v[x as usize];
                memory.set_from_index(self.i as usize, v / 100);
                memory.set_from_index(self.i as usize + 1, (v % 100) / 10);
                memory.set_from_index(self.i as usize  + 2, v % 10);
                self.pc += 2;
            },
            Instructions::Instruction3xnn => {
                let nn = opcode & 0x00ff;
                if self.v[x as usize] == nn as u8 { self.pc += 4; } else { self.pc += 2; }
            }
            Instructions::Instruction4xnn => {
                let nn = opcode & 0x00ff;
                if self.v[x as usize] != nn as u8 { self.pc += 4; } else { self.pc += 2; }
            },
            Instructions::Instruction5xy0 => {
                if self.v[x as usize] == self.v[y as usize] { self.pc += 4; } else { self.pc += 2; }
            },
            Instructions::Instruction9xy0 => {
                if self.v[x as usize] != self.v[y as usize] { self.pc += 4; } else { self.pc += 2; }
            },
            Instructions::Instruction00ee => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy2 => {
                self.v[x as usize] &= self.v[y as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy1 => {
                self.v[x as usize] |= self.v[y as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy3 => {
                self.v[x as usize] ^= self.v[y as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy4 => {
                let result = u16::from(self.v[x as usize]).wrapping_add(self.v[y as usize] as u16);
                self.v[x as usize] = (result & 0xFF) as u8; // truncate
                self.v[0xF] = if result > 0xFF { 1 } else { 0 };
                self.pc += 2;
            },
            Instructions::Instruction8xy0 => {
                self.v[x as usize] = self.v[y as usize];
                self.pc += 2;
            },
            Instructions::Instruction8xy5 => {
                let flag = self.v[x as usize] >= self.v[y as usize];
                self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
                self.v[0xF] = flag as u8;
                self.pc += 2;
            },
            Instructions::Instruction8xy7 => {
                let flag = self.v[y as usize] >= self.v[x as usize];
                self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);
                self.v[0xF] = flag as u8;
                self.pc += 2;
            },
            Instructions::Instruction8xy6 => {
                let flag = self.v[y as usize] & 0b00000001 > 0;
                self.v[x as usize] = self.v[y as usize] >> 1;
                self.v[0xF] = flag as u8;
                self.pc += 2;
            },
            Instructions::Instruction8xye => {
                let flag = self.v[y as usize] & 0b10000000 > 0;
                self.v[x as usize] = self.v[y as usize] << 1;
                self.v[0xF] = flag as u8;
                self.pc += 2;
            },
            Instructions::Instructionfx55 => {
                for i in 0..=x {
                    memory.set_from_index(self.i as usize + i as usize, self.v[i as usize]);
                }
                self.i += x + 1; // do not use this unless you strictly use CHIP-8
                self.pc += 2;
            },
            Instructions::Instructionfx65 => {
                for i in 0..=x {
                    self.v[i as usize] = memory.get_from_index(self.i as usize + i as usize);
                }
                self.i += x + 1; // do not use this unless you strictly use CHIP-8
                self.pc += 2;
            },
            Instructions::Instructionfx29 => {
                self.i = (self.v[x as usize] * 5) as u16;
                self.pc += 2;
            },
            Instructions::Instructioncxnn => {
                let nn = opcode & 0x00ff;
                // I suppose this will suffice, honestly.
                self.v[x as usize] = (rand::thread_rng().gen::<u16>() & nn) as u8;
                self.pc += 2;
            },
            Instructions::Instructionbnnn => {
                let nnn = opcode & 0x0fff;
                self.pc = nnn + self.v[x as usize] as u16; // v[0] if you want XO-CHIP / SUPER-CHIP
                self.pc += 2;
            },
            Instructions::Instructionfx1e => {
                self.i += self.v[x as usize] as u16;
                self.v[0xF] = if self.i > 0x0F00 { 1 } else { 0 };
                self.pc += 2;
            },
            Instructions::Instructionfx07 => {
                self.v[x as usize] = self.delay_timer;
                self.pc += 2;
            },
            Instructions::Instructionfx15 => {
                self.delay_timer = self.v[x as usize];
                self.pc += 2;
            },
            Instructions::Instructionfx18 => {
                self.sound_timer = self.v[x as usize];
                self.pc += 2;
            },
            Instructions::Instructionfx0a => {

                // let execution_paused: bool = self.get_flags() & 0xA != 0;

                // pause execution
                // if !execution_paused { self.flags |= 0xA; }

                let (key_code, pressed) = keypad.is_any_key_down_emulator();

                if pressed {
                    self.v[x as usize] = key_code;
                    self.pc += 2;
                    // resume execution
                    // self.flags &= 0xA;
                } else {
                    self.pc -= 2;
                }
            },
            Instructions::Instructionexa1 => {
                let pressed = keypad.is_key_down_emulator(self.v[x as usize]);
                if !pressed { self.pc += 2; }
                self.pc += 2;
            },
            Instructions::Instructionex9e => {
                let pressed = keypad.is_key_down_emulator(self.v[x as usize]);
                if pressed { self.pc += 2; }
                self.pc += 2;
            }
        }
    }

    fn fetch(&self, memory: &mut Memory) -> u16 {
        let part1 = memory.get_from_index(self.pc as usize) as u16;
        let part2 = memory.get_from_index((self.pc + 1) as usize) as u16;
        return part1 << 8 | part2;
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
                    0x006 => {
                        Instructions::Instruction8xy6
                    },
                    0x0007 => {
                        Instructions::Instruction8xy7
                    },
                    0x00e => {
                        Instructions::Instruction8xye
                    }
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
            0xB000 => {
                Instructions::Instructionbnnn
            },
            0xC000 => {
                Instructions::Instructioncxnn
            }
            0xD000 => {
                Instructions::Instructiondxyn
            },
            0xE000 => {
                match opcode & 0x00ff {
                    0x009E => {
                        Instructions::Instructionex9e
                    },
                    0x00A1 => {
                        Instructions::Instructionexa1
                    },
                    _ => {
                        panic!("Unknown Instruction (0xE000 family), opcode: 0x{:X}", opcode);
                    }
                }
            },
            0xF000 => {
                match opcode & 0x00ff {
                    0x0033 => {
                        Instructions::Instructionfx33
                    },
                    0x0029 => {
                        Instructions::Instructionfx29
                    },
                    0x0055 => {
                        Instructions::Instructionfx55
                    },
                    0x0065 => {
                        Instructions::Instructionfx65
                    },
                    0x001E => {
                        Instructions::Instructionfx1e
                    },
                    0x000A => {
                        Instructions::Instructionfx0a
                    },
                    0x0007 => {
                        Instructions::Instructionfx07
                    },
                    0x0015 => {
                        Instructions::Instructionfx15
                    },
                    0x0018 => {
                        Instructions::Instructionfx18
                    },
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
    pub fn get_delay_timer(&self) -> u8 {
        return self.delay_timer;
    }
    pub fn get_sound_timer(&self) -> u8 {
        return self.sound_timer;
    }
    pub fn dec_delay_timer(&mut self) {
        self.delay_timer -= 1;
    }
    pub fn dec_sound_timer(&mut self) {
        self.sound_timer -= 1;
    }
}