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

use std::thread;
use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;

mod interpreter;

const PROCESSOR_CLOCK_SPEED: u64 = 500; // 500 Mhz default, configured via option.

// TODO refactor
struct ConsoleArgs {
    path: String,
}

fn main() {
    
    let context = sdl2::init().unwrap();
    let mut events = context.event_pump().unwrap();

    // create CPU instance
    let mut cpu = interpreter::cpu::CPU::new();
    // create Keypad instance
    let mut keypad = interpreter::keypad::Keypad::new();
    // create Memory instance
    let mut memory = interpreter::memory::Memory::new();
    // create Display instance
    let mut display = interpreter::display::Display::new(&context);

    // copy font to memory
    memory.initialize();

    let path = std::env::args().nth(1).expect("No ROM file found.");
    // load rom
    memory.load_rom(&path);

    // NOTE: this is not a proper way to emulate the processor speed and 500 MHz is arbitrary value.
    // This part of the code should be completely rewritten while counting the time taken for each instruction
    // And then properly adjusting the speed, but that's for another day.
    // Source: https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html
    let interval = Duration::from_secs_f32(1.0 / PROCESSOR_CLOCK_SPEED as f32);

    let mut division_cycles: u32 = 0;
    
    'running: loop {

        let start_time = Instant::now();
        
        cpu.execute(&mut display, &mut memory, &mut keypad);
        division_cycles += 1;
        
        let elapsed = Instant::now() - start_time;
        if elapsed < interval {
            thread::sleep(interval - elapsed);
        }

        for event in events.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            };
        }
    
        // Check for new inputs
        keypad.is_key_down(&events);

        // (500Hz / 60Hz) == ~9 Cycles
        if division_cycles == 9
        {
            if cpu.get_delay_timer() > 0
            {
                cpu.dec_delay_timer();
            }

            if cpu.get_sound_timer() > 0
            {
                // NOTE: Will only work on *NIX
                // std::process::Command::new("echo \"\x07\"").exec();
                cpu.dec_sound_timer();
            }

            division_cycles = 0; // reset cycle count.
        }
    }
}
