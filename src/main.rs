use std::time::Duration;
use sdl2::event::Event;
mod interpreter;

#[cfg(test)]
mod tests;

fn main() {
    
    let mut cpu = interpreter::cpu::CPU::new(); // create new instance of CPU
    cpu.initialize(); // initialize CPU

    let mut memory = interpreter::memory::Memory::new();
    memory.initialize();
    memory.load_rom("ibm.ch8");

    let context = sdl2::init().unwrap();
    let mut display = interpreter::display::Display::new(&context);

    let mut event_pump = context.event_pump().unwrap();
    'running: loop {

        cpu.execute(&mut memory);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }

        {
            if cpu.get_flags() & 0xA != 0 {
                print!("CPU is requesting render!\n");
                let buffer = cpu.get_cpu_buffer();
                // let sss: String = buffer.iter().map(|&x| format!("{:?}", x)).collect();
                // print!("bf is\n{}\n", sss);
                display.draw(buffer, 10, 10);
                cpu.remove_flag(0xA);
            }
        }

       std::thread::sleep(Duration::from_millis(1000));
    }
}
