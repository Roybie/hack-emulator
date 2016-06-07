const ROM_SIZE: usize = 32768;

pub struct Rom {
    rom: Box<[u16]>
}

impl Rom {
    pub fn new(new_rom: Box<[u16]>) -> Rom {
        Rom {
            rom: new_rom,
        }
    }

    pub fn get_instruction(&self, address: usize) -> u16 {
        if address < ROM_SIZE {
            self.rom[address]
        } else {
            panic!("Invalid ROM address: {:#x}", address)
        }
    }
}
