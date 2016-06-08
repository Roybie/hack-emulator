//const ROM_SIZE: usize = 32768;

pub struct Rom {
    rom: Box<[i16]>
}

impl Rom {
    pub fn new(new_rom: Box<[i16]>) -> Rom {
        Rom {
            rom: new_rom,
        }
    }

    pub fn get_instruction(&self, address: usize) -> i16 {
        if address < self.rom.len() {
            self.rom[address]
        } else {
            panic!("Invalid ROM address: {:#x}", address)
        }
    }
}
