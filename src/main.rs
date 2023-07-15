use std::thread;
use std::time::Duration;
use sdl2::event::Event;

mod interpreter;

#[cfg(test)]
mod tests;

fn main() {
    
    let mut cpu = interpreter::cpu::CPU::new(); // create new instance of CPU

    let mut memory = interpreter::memory::Memory::new();
    memory.initialize();
    memory.load_rom("roms/test_opcode.ch8");

    let context = sdl2::init().unwrap();
    let mut display = interpreter::display::Display::new(&context);

    let mut event_pump = context.event_pump().unwrap();
    'running: loop {

        cpu.execute(&mut memory);
        // cpu.print_debug();

        if cpu.get_flags() & 0xF != 0 {
            let buffer = cpu.get_buffer();
            display.draw(buffer);
            cpu.remove_flag(0xF);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
