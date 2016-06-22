const RAM_SIZE: usize = 16 * 1024;
const SCREEN_SIZE: usize = 256 * 512 * 3;

const SCREEN_ADDR: usize = 0x4000;
const KEYBD_ADDR: usize = 0x6000;

pub struct Memory {
    ram: Box<[i16]>,
    cpu_screen: Box<[i16]>,
    screen: Box<[u8]>,
    keyboard: i16,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            cpu_screen: vec![0; 512 * 16].into_boxed_slice(),
            screen: vec![0xff; SCREEN_SIZE].into_boxed_slice(),
            keyboard: 0,
        }
    }

    pub fn write_memory(&mut self, address: usize, value: i16) {
        if address < SCREEN_ADDR {
            self.ram[address] = value;
        } else if address < KEYBD_ADDR {
            self.cpu_screen[address - SCREEN_ADDR] = value;
            let mut ind = (address - SCREEN_ADDR) * (16 * 3);
            let mut mask = 0b1;
            for _ in 0..16 {
                let v = match value & mask {
                    0 => 0xff,
                    _ => 0,
                };
                self.screen[ind] = v;
                self.screen[ind + 1] = v;
                self.screen[ind + 2] = v;
                ind += 3;
                mask = mask << 1;
            }
        } else if address == KEYBD_ADDR {
            self.keyboard = value;
        } else {
            panic!("Invalid memory address: {:#x}", address);
        }
    }
    pub fn read_memory(&self, address: usize) -> i16 {
        if address < SCREEN_ADDR {
            self.ram[address]
        } else if address < KEYBD_ADDR {
            self.cpu_screen[address - SCREEN_ADDR] as i16
        } else if address == KEYBD_ADDR {
            self.keyboard
        } else {
            panic!("Invalid memory address: {:#x}", address)
        }
    }

    pub fn get_screen(&self) -> &Box<[u8]> {
        &self.screen
    }

    pub fn set_key(&mut self, key: i16) {
        //map hack special keycodes from sdl
        self.keyboard = match key {
            13 | 88     => 128,     //newline
            8           => 129,     //backspace
            80          => 130,     //left arrow
            82          => 131,     //up arrow
            79          => 132,     //right arrow
            81          => 133,     //down arrow
            74          => 134,     //home
            77          => 135,     //end
            75          => 136,     //page up
            78          => 137,     //page down
            73          => 138,     //insert
            127         => 139,     //delete
            27          => 140,     //esc
            58          => 141,     //f1
            59          => 142,
            60          => 143,
            61          => 144,
            62          => 145,
            63          => 146,
            64          => 147,
            65          => 148,
            66          => 149,
            67          => 150,
            68          => 151,
            69          => 152,     //f12
            _           => key,     //other keys
        };
    }
}
