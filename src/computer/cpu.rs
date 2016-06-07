use super::{ Memory, Rom };

#[derive(Default)]
pub struct Cpu {
    reg_a: u16,
    reg_d: u16,
    reg_pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }

    pub fn execute_instruction(&mut self, rom: &Rom, memory: &mut Memory) {
        let instruction: u16 = rom.get_instruction(self.reg_pc as usize);
        let op = (instruction & 0x8000) >> 15;

        match op {
            0 => self.a_instruction(instruction),
            _ => self.c_instruction(instruction, memory),
        };
    }

    pub fn a_instruction(&mut self, instruction: u16) {
        self.reg_a = instruction;

    }
    pub fn c_instruction(&mut self, instruction: u16, memory: &mut Memory) {
        let jump = instruction & 0b111;
        let dest_m = (instruction & 0b1000) >> 3;
        let dest_d = (instruction & 0b10000) >> 4;
        let dest_a = (instruction & 0b100000) >> 5;
        let comp = (instruction & 0b1111111000000) >> 6;

        //TODO
        let result = match comp {
            _ => 1,
        };

        if dest_m == 1 {
            memory.write_memory(self.reg_a as usize, result);
        }
        if dest_d == 1 {
            self.reg_d = result;
        }
        if dest_a == 1 {
            self.reg_a = result;
        }

        match jump {
            _ => self.reg_pc = self.reg_a,
        }
    }
}
