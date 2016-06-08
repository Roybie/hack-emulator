extern crate byteorder;
extern crate sdl;
extern crate time;

use byteorder::{LittleEndian, ReadBytesExt };

use std::env;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;

use sdl::event::Event;
use sdl::Rect;
use sdl::video;

use computer::Computer;

mod computer;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_bin(rom_file_name);

    let cycle_time = 2000000; // nanoseconds 500hz

    let two: i16 = 2;

    let mut hack = Computer::new(rom);

    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Timer]);

    let screen = video::set_video_mode(512, 256, 8, &[video::SurfaceFlag::HWSurface], &[video::VideoFlag::DoubleBuf]).unwrap();

    'main : loop {

        let start_time = time::precise_time_ns();

        //check keyboard and update keyboard memory map
        'key : loop {
            match sdl::event::poll_event() {
                Event::Quit                     => break 'main,
                Event::None                     => break 'key,
                Event::Key(key, state, _, _)    => hack.set_key(key as i16, state),
                _                               => (),
            }
        }

        //hack.tick();

        //get screen and draw
        screen.fill_rect(Some(Rect{x: 0, y: 0, w: 512, h: 256}), video::RGB(255, 255, 255));
        for (i, mem) in hack.get_screen().into_iter().enumerate() {
            for j in 0..16 {
                if mem & two.pow(j as u32) != 0 {
                    let y = i / 32;
                    let x = j + (i * 16) % 512;
                    screen.fill_rect(Some(Rect{x: x as i16, y: y as i16, w: 1, h: 1}), video::RGB(0, 0, 0));
                }
            }

        }
        screen.flip();

        //sleep to simulate clock speed
        let remaining_time = cycle_time - (time::precise_time_ns() - start_time);
        if remaining_time > 0 && remaining_time < cycle_time {
            thread::sleep(Duration::new(0, remaining_time as u32));
        }
    }
    sdl::quit();
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
