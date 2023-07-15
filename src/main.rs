use std::time::Duration;

use interpreter::memory::Memory;
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

    let mut screen = interpreter::screen::Screen::new();

    let context = sdl2::init().unwrap();

    let window = context.video().unwrap().window("CHIP-8 Interpreter", 1280, 640).opengl().allow_highdpi().position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture(sdl2::pixels::PixelFormatEnum::RGBA8888, sdl2::render::TextureAccess::Target, 64, 32).unwrap();

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
                // let screen = &mut screen;
                print!("CPU is requesting render!\n");
                let cpu_buffer = cpu.get_cpu_buffer();
                let buffer = screen.convert_buffer(cpu_buffer);
                let rect = sdl2::rect::Rect::new(0, 0, 64, 32);
                texture.update(rect, &buffer, (4 * 64) as usize).unwrap();
                let rect = sdl2::rect::Rect::new(0, 0, 64, 32);
                canvas.copy(&texture, None, Some(rect)).unwrap();
                canvas.present();
                // cpu.remove_flag(0xA);
            }
        }

        std::thread::sleep(Duration::from_millis(1000));
    }
}
