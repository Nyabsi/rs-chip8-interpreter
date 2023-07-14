use std::time::Duration;

use interpreter::memory::Memory;

mod interpreter;

#[cfg(test)]
mod tests;

fn main() {
    
    let mut cpu = interpreter::cpu::CPU::new(); // create new instance of CPU
    cpu.initialize(); // initialize CPU

    let memory = cpu.get_memory();

    memory.initialize();
    memory.load_rom("ibm.ch8");

    loop {
        cpu.execute();
        std::thread::sleep(Duration::from_millis(1000));
    }
}
