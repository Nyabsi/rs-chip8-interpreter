// This file manages the memory of the interpreter.
use std::fs::File;
use std::io::Read;
use core::ascii;
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

pub struct Memory {
    ram: [u8; 4096],
    rom_loaded: bool,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: [0; 4096],
            rom_loaded: false,
        }
    }
    pub fn initialize(&mut self) {
        // copy FONTSET to ram.
        self.ram[0..FONTSET.len()].copy_from_slice(&FONTSET);
    }
    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to load ROM from file, make sure the path is valid.");
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).expect("Failed ROM from file, aborting!");
        if (0x200 + 0xFFF) >= buffer.len() {
            // copy rom to 0x200 in our memory.
            self.ram[0x200..0x200+buffer.len()].copy_from_slice(&buffer);
        } else {
            panic!("Failed to copy ROM to memory, exceeding maximum allowed ROM size (4096 bytes)");
        }
        self.rom_loaded = true;
        print!("ROM loaded!\n");
    }
    // I am adding this to validate the memory actually loads properly.
    pub fn dump_memory(&mut self) -> String {
        return self.ram.iter().map(|&x| format!("{:02X}", x)).collect();
    }
    // TODO: similar function for writing memory, not required yet.
    pub fn get_from_index(&mut self, i: usize) -> u8 {
        return *self.ram.index(i);
    }
}