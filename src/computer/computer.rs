use sdl::event::Key;

use super::{ Cpu, Memory, Rom };

pub struct Computer {
    cpu: Cpu,
    memory: Memory,
    rom: Rom,
}

impl Computer {
    pub fn new(rom: Box<[i16]>) -> Computer {
        Computer {
            cpu: Cpu::new(),
            memory: Memory::new(),
            rom: Rom::new(rom),
        }
    }
    pub fn tick(&mut self) {
        self.cpu.execute_instruction(&self.rom, &mut self.memory);
    }

    pub fn set_key(&mut self, key: i16, state: bool) {
        self.memory.set_key(match state { true => key, false => 0 });
    }

    pub fn get_screen(&self) -> &[i16] {
        self.memory.get_screen()
    }
}
