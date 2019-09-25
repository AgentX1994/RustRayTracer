use sdl2::pixels;

#[derive(Debug)]
pub struct Material {
    pub color: pixels::Color,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: pixels::Color::RGBA(255, 255, 255, 255),
        }
    }
}
