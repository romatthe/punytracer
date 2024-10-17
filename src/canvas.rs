use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn save_to(&self, path: &impl AsRef<Path>) {
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        let pixel_data = self.pixels.iter()
            .flat_map(|c| c.to_rgb())
            .collect::<Vec<u8>>();
        writer.write_image_data(&pixel_data).unwrap()
    }
}

impl std::ops::Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[self.width * y + x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[self.width * y + x]
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        // Given
        let canvas = Canvas::new(10, 20);

        // Then
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert!(canvas.pixels.iter().all(|c| *c == Color::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn writing_pixels_to_canvas() {
        // Given
        let mut canvas = Canvas::new(10, 20);

        // When
        canvas[(1,2)] = Color::new(1.0, 0.0, 0.0);

        // Then
        assert_eq!(canvas[(1,2)], Color::new(1.0, 0.0, 0.0));
    }
}