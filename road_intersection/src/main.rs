// main.rs

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


mod map;
use map::Map;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window_builder = video_subsystem.window("Map Example", 800, 600);
    window_builder.fullscreen_desktop();

    let window = window_builder.build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let (width, height) = canvas.output_size().unwrap();

    let map = Map::new(width, height);
    map.draw(&mut canvas, width as i32, height as i32);

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
    }
}
