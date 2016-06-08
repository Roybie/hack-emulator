mod computer;

extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt };
use std::env;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;

use computer::Computer;

fn main() {
    let clock_delay = Duration::from_millis(2);

    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_bin(rom_file_name);

    let mut hack = Computer::new(rom);

    loop {
        //check keyboard and update keyboard memory map
        hack.get_keys();

        hack.tick();

        //get screen and draw
        hack.get_screen();

        //sleep to simulate clock speed
        thread::sleep(clock_delay);
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[i16]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    loop {
        let next_i16 = match file.read_i16::<LittleEndian>() {
            Ok(n) => n,
            Err(_) => break,
        };
        file_buf.push(next_i16);
    }
    file_buf.into_boxed_slice()
}
