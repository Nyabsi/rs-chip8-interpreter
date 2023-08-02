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

use std::fs::File;
use std::io::Read;
use std::ops::Index;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const ROM_OFFSET: usize = 0x200;
const ROM_OFFSET_MAX: usize = 0xFFF;

pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: [0; 4096],
        }
    }

    pub fn initialize(&mut self) {
        // copy FONTSET to ram at FONTSET (0x50).
        self.ram[0..FONTSET.len()].copy_from_slice(&FONTSET);
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to load ROM from file, make sure the path is valid.");
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).expect("Failed ROM from file, aborting!");
        // If the ROM goes beyond unallowed memory region, we'll panic.
        if (ROM_OFFSET + ROM_OFFSET_MAX) >= buffer.len() {
            // Load the program at 0x200 that's where we'll begin execution.
            self.ram[ROM_OFFSET..ROM_OFFSET+buffer.len()].copy_from_slice(&buffer);
        } else {
            panic!("Failed to copy ROM to memory, exceeding maximum allowed ROM size (4096 bytes)");
        }
        println!("Loaded ROM, Size: {:?} Bytes.", buffer.len());
    }

    pub fn get_from_index(&mut self, i: usize) -> u8 {
        return *self.ram.index(i);
    }

    pub fn set_from_index(&mut self, i: usize, data: u8) {
        self.ram[i] = data;
    }
}