extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;

use std::f64::consts::PI;

pub mod ray;
pub mod sphere;
pub mod vector3d;

use sphere::Sphere;
use ray::Ray;
use vector3d::Vector3;

#[derive(PartialEq)]
enum ProjectionMode {
    Ortho,
    Perspective
}

fn create_window(context: &sdl2::Sdl, title: &str, width: u32, height: u32) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError>{
    let video_subsystem = context.video().unwrap();

    video_subsystem.window(title, width, height)
        .position_centered()
        .opengl()
        .build()
}

pub fn run(width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let window = create_window(&sdl_context, "Ray Tracer", width, height).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGBA(0, 0, 0, 255));
    // Create texture to draw onto (to avoid double buffer problem)
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(pixels::PixelFormatEnum::RGBA8888, width, height)
        .unwrap();

    // Clear texture
    canvas.with_texture_canvas(&mut texture, |texture_canvas| {
        texture_canvas.clear();
    }).unwrap();

    // Set up sphere model
    let sphere = Sphere::new(
        Vector3::new(0.0, 0.0, -3.0),
        1.0,
    );

    let mut fovy = 90.0; // Degrees
    let mut fovx = 90.0; // Degrees

    let camera_pos = Vector3::new(0.0, 0.0, 0.0);
    let camera_dir = Vector3::new(0.0, 0.0, -1.0);

    let sphere_color = pixels::Color::RGBA(255, 255, 255, 255);
    let blank_color = pixels::Color::RGBA(0, 0, 0, 255);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut mode = ProjectionMode::Perspective;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::P => if mode == ProjectionMode::Ortho { mode = ProjectionMode::Perspective } else { mode = ProjectionMode::Ortho },
                        Keycode::Up => {
                            fovy += 1.0;
                            println!("fovy = {}", fovy);
                        },
                        Keycode::Down => {
                            fovy -= 1.0;
                            println!("fovy = {}", fovy);
                        },
                        Keycode::Right => {
                            fovx += 1.0;
                            println!("fovx = {}", fovx);
                        },
                        Keycode::Left => {
                            fovx -= 1.0;
                            println!("fovx = {}", fovx);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        // Ray Trace!
        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            for dx in 0..width {
                for dy in 0..height {

                    let pos = camera_pos.clone();
                    let dir = match mode {
                        ProjectionMode::Ortho => {
                            let x = ((dx as f64) - (width as f64)/2.0)/(width as f64);
                            let y = ((dy as f64) - (height as f64)/2.0)/(height as f64);
                            let view_plane_pos = Vector3::new(x, y, 0.0).add(&camera_dir);
                            view_plane_pos.into_unit()
                        },
                        ProjectionMode::Perspective => {
                            // from https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays
                            let pixel_x_ndc = (dx as f64 + 0.5f64)/width as f64;
                            let pixel_y_ndc = (dy as f64 + 0.5f64)/height as f64;

                            let pixel_screen_x = 2.0*pixel_x_ndc - 1.0;
                            let pixel_screen_y = 2.0*pixel_y_ndc - 1.0;

                            //println!("Now rendering screen coords ({}, {})", pixel_screen_x, pixel_screen_y);

                            const DEGREES_TO_RADIANS : f64 = PI/180.0;
                            let aspect_ratio = width as f64 / height as f64;
                            let pixel_camera_x = pixel_screen_x * aspect_ratio * (fovx/2.0 as f64 * DEGREES_TO_RADIANS).tan();
                            let pixel_camera_y = pixel_screen_y * (fovy/2.0 as f64 * DEGREES_TO_RADIANS).tan();
                            let pixel_camera_space = Vector3::new(pixel_camera_x, pixel_camera_y, -1.0);
                            //println!("\tfinal direction vec (before normalization): {:?}", pixel_camera_space);
                            pixel_camera_space.into_unit()

                            // TODO in case of moving camera, make sure to transform this point
                            // into world space before normalizing it!
                        }
                    };


                    let r = Ray::new(pos, dir);

                    if let Some((t0, t1)) = sphere.ray_intersection(&r) {
                        let t = if t1 < 0.0 {
                            t0
                        } else {
                            t1
                        };
                        let p = r.pos.add(&(r.dir.mul(t)));
                        let normal = sphere.pos.sub(&p).into_unit();
                        let view = p.sub(&camera_pos).into_unit();
                        let mut proportion = normal.dot(&view);
                        let (mut red, mut green, mut blue) = sphere_color.rgb();
                        if proportion < 0.0 {
                            proportion = 0.0;
                        }
                        red = ((red as f64) * proportion) as u8;
                        blue = ((blue as f64) * proportion) as u8;
                        green = ((green as f64)* proportion) as u8;
                        let color = pixels::Color::RGB(red, green, blue);
                        texture_canvas.pixel(dx as i16, dy as i16, color).unwrap();
                    } else {
                        texture_canvas.pixel(dx as i16, dy as i16, blank_color).unwrap();
                    }
                }
            }
        }).unwrap();

        // The rest of the game loop goes here
        canvas.clear();
        canvas
            .copy_ex(
                &texture,
                None,       // Source Rect (None = whole texture)
                None,       // Destination Rect (None = whole canvas)
                0.0,        // rotation angle
                None,       // Center of Rotation (None = center of dst, or src if dst is None)
                false,      // flip horizontal?
                false)      // flip vertical?
            .unwrap();
        canvas.present();
    }
}
