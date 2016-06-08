use super::{ Memory, Rom };

enum Jump {
    Zero,
    Greater,
    Less,
}

impl Jump {
    pub fn get(val:i16) -> Jump {
        match val {
            0 => Jump::Zero,
            _ if val > 0 => Jump::Greater,
            _ => Jump::Less,
        }
    }
}

#[derive(Default)]
pub struct Cpu {
    reg_a: i16,
    reg_d: i16,
    reg_pc: i16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }

    pub fn execute_instruction(&mut self, rom: &Rom, memory: &mut Memory) {
        let instruction: i16 = rom.get_instruction(self.reg_pc as usize);
        let op: bool = (instruction >> 15) == 1;

        match op {
            false => self.a_instruction(instruction),
            true => self.c_instruction(instruction, memory),
        };
    }

    pub fn a_instruction(&mut self, instruction: i16) {
        self.reg_a = instruction;
        self.reg_pc += 1;
    }

    #[allow(overflowing_literals)]
    pub fn c_instruction(&mut self, instruction: i16, memory: &mut Memory) {
        let mut jump = (instruction & 0b111, Jump::Zero);
        let dest_m = (instruction & 0b1000) >> 3;
        let dest_d = (instruction & 0b10000) >> 4;
        let dest_a = (instruction & 0b100000) >> 5;
        let comp = (instruction & 0b1111111000000) >> 6;

        let reg_m:i16 = memory.read_memory(self.reg_a as usize);

        let result:i16 = match comp {
            0b0101010 => 0,
            0b0111111 => 1,
            0b0111010 => -1,
            0b0001100 => self.reg_d,
            0b0110000 => self.reg_a,
            0b1110000 => reg_m,
            0b0001101 => 0xFFFF ^ self.reg_d,
            0b0110001 => 0xFFFF ^ self.reg_a,
            0b1110001 => 0xFFFF ^ reg_m,
            0b0011111 => self.reg_d + 1,
            0b0110111 => self.reg_a + 1,
            0b1110111 => reg_m + 1,
            0b0001110 => self.reg_d - 1,
            0b0110010 => self.reg_a - 1,
            0b1110010 => reg_m - 1,
            0b0000010 => self.reg_d + self.reg_a,
            0b1000010 => self.reg_d + reg_m,
            0b0010011 => self.reg_d - self.reg_a,
            0b1010011 => self.reg_d - reg_m,
            0b0000111 => self.reg_a - self.reg_d,
            0b1000111 => reg_m - self.reg_d,
            0b0000000 => self.reg_d & self.reg_a,
            0b1000000 => self.reg_d & reg_m,
            0b0010101 => self.reg_d | self.reg_a,
            0b1010101 => self.reg_d | reg_m,
            _ => panic!("Unknown computation {:#b}", comp),
        };

        jump.1 = Jump::get(result);

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
            (0b001, Jump::Greater)  |
            (0b010, Jump::Zero)     |
            (0b011, Jump::Greater)  |
            (0b011, Jump::Zero)     |
            (0b100, Jump::Less)     |
            (0b101, Jump::Greater)  |
            (0b101, Jump::Less)     |
            (0b110, Jump::Less)     |
            (0b110, Jump::Zero)     |
            (0b111, _) => self.reg_pc = self.reg_a,
            _ => self.reg_pc += 1,
        }
    }
}
