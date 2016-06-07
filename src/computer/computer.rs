use super::{ Cpu, Memory, Rom };

pub struct Computer {
    cpu: Cpu,
    memory: Memory,
    rom: Rom,
}

impl Computer {
    pub fn new(rom: Box<[u16]>) -> Computer {
        Computer {
            cpu: Cpu::new(),
            memory: Memory::new(),
            rom: Rom::new(rom),
        }
    }
    pub fn tick(&mut self) {
        self.cpu.execute_instruction(&self.rom, &mut self.memory);
        self.memory.write_memory(0, 0xabc);
    }
    pub fn get_mem(&self, address: usize) -> u16 {
         self.memory.read_memory(address)
    }
}
