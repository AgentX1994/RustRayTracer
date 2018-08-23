extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;

pub fn run(width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("rust-sdl2 demo", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if keycode == Keycode::Escape {
                        break 'main
                    } else if keycode == Keycode::Space {
                        println!("space down");
                        for i in 0..400 {
                            canvas.pixel(i as i16, i as i16, 0xFF000Fu32).unwrap();
                        }
                        canvas.present();
                    }
                }

                Event::MouseButtonDown { x, y, ..} => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
        // The rest of the game loop goes here
        
    }
}
