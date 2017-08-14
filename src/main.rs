extern crate sdl2;
extern crate rrasterizer;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rrasterizer::application::Application;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut application = Application::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        application.render();

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| for y in 0..
                WINDOW_HEIGHT as usize
            {
                for x in 0..WINDOW_WIDTH as usize {
                    let (r, g, b) = application.get_pixel(x as u32, WINDOW_HEIGHT - y as u32 - 1);

                    let offset = y * pitch + x * 3;
                    buffer[offset] = r;
                    buffer[offset + 1] = g;
                    buffer[offset + 2] = b;
                }
            })
            .unwrap();

        canvas.clear();
        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)),
            )
            .unwrap();
        canvas.present();
    }
}
