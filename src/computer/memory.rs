const RAM_SIZE: usize = 16 * 1024;
const SCREEN_SIZE: usize = 16 * 512;

const SCREEN_ADDR: usize = 0x4000;
const KEYBD_ADDR: usize = 0x6000;

pub struct Memory {
    ram: Box<[u16]>,
    screen: Box<[u16]>,
    keyboard: u16,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            screen: vec![0; SCREEN_SIZE].into_boxed_slice(),
            keyboard: 0,
        }
    }
    pub fn write_memory(&mut self, address: usize, value: u16) {
        if address < SCREEN_ADDR {
            self.ram[address] = value;
        } else if address < KEYBD_ADDR {
            self.screen[address - SCREEN_ADDR] = value;
        } else if address == KEYBD_ADDR {
            self.keyboard = value;
        } else {
            panic!("Invalid memory address: {:#x}", address);
        }
    }
    pub fn read_memory(&self, address: usize) -> u16 {
        if address < SCREEN_ADDR {
            self.ram[address]
        } else if address < KEYBD_ADDR {
            self.screen[address - SCREEN_ADDR]
        } else if address == KEYBD_ADDR {
            self.keyboard
        } else {
            panic!("Invalid memory address: {:#x}", address)
        }
    }
}
