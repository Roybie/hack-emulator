extern crate sdl2;
extern crate time;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
//use std::thread;

use sdl2::render::{TextureAccess};
use sdl2::pixels::PixelFormatEnum::BGR24;
use sdl2::event::Event;

use computer::Computer;

mod computer;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_bin(rom_file_name);

    let refresh_rate = 0.02;
    let scale: u32 = 2;
    let res = (512, 256);

    let mut hack = Computer::new(rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window_builder = video_subsystem.window("HACK emulator",
                                        res.0 * scale,
                                        res.1* scale);
    let window = window_builder.position_centered().opengl().build().unwrap();

    let mut renderer = window.renderer().accelerated().present_vsync().build().unwrap();

    let mut texture = renderer.create_texture(
            BGR24,
            TextureAccess::Streaming,
            res.0,
            res.1
        ).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut start_time = time::precise_time_s();
    let mut new_time: f64 = 0.0;

    'main : loop {

        new_time += time::precise_time_s() - start_time;
        start_time = time::precise_time_s();
        if new_time >= refresh_rate {
            new_time -= refresh_rate;
            texture.update(None, hack.get_screen(), res.0 as usize * 3).unwrap();
            renderer.copy(&texture, None, None);
            renderer.present();
        }

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
        hack.tick();

        //sleep to simulate clock speed
        //let remaining_time = cycle_time - (time::precise_time_ns() - start_time);
        //if remaining_time > 0 && remaining_time < cycle_time {
            //thread::sleep(Duration::new(0, remaining_time as u32));
        //}
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[i16]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    for line in string.lines() {
       file_buf.push(u16::from_str_radix(line, 2).unwrap() as i16);
    }
    file_buf.into_boxed_slice()
}
