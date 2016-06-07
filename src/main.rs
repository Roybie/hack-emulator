mod computer;

use std::env;

use computer::Computer;

fn main() {
    let rom = vec![0;32768].into_boxed_slice(); //env::args().nth(1).unwrap();

    let mut hack = Computer::new(rom);

    hack.tick();

    let mem = hack.get_mem(0x0000);
    println!("{}", mem);
}
