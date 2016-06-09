extern crate byteorder;
extern crate sdl2;
extern crate time;

use byteorder::{LittleEndian, ReadBytesExt };

use std::env;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use computer::Computer;

mod computer;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_bin(rom_file_name);

    let cycle_time = 2000000; // nanoseconds, 500hz
    let scale: u32 = 2;
    let res = (512 * scale, 256 * scale);

    let mut hack = Computer::new(rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("HACK computer", res.0, res.1)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'main : loop {

        let start_time = time::precise_time_ns();

        //check keyboard and update keyboard memory map
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(key), .. } => hack.set_key(key as i16, true),
                Event::KeyUp { .. } => hack.set_key(0, false),
                _ => (),
            }
        }

        //Perform next cpu step
        //hack.tick();

        //get screen and draw
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(0, 0, 0));

        for (i, mem) in hack.get_screen().into_iter().enumerate() {
            if *mem != 0 {
                if *mem == -1 {
                    let y = scale as usize * (i / 32);
                    let x = scale as usize * ((i * 16) % 512);
                    renderer.fill_rect(Rect::new(x as i32, y as i32, scale * 16, scale));
                } else {
                    let mut pixel = *mem;
                    for j in 0..16 {
                        if pixel & 1 != 0 {
                            let y = scale as usize * (i / 32);
                            let x = scale as usize * (j + (i * 16) % 512);
                            renderer.fill_rect(Rect::new(x as i32, y as i32, scale, scale));
                        }
                        pixel = pixel >> 1;
                    }
                }
            }
        }
        renderer.present();

        //sleep to simulate clock speed
        let remaining_time = cycle_time - (time::precise_time_ns() - start_time);
        if remaining_time > 0 && remaining_time < cycle_time {
            thread::sleep(Duration::new(0, remaining_time as u32));
        }
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
