use std::fs::File;

use image::{ExtendedColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use indicatif::ProgressBar;
use nalgebra_glm::Vec3;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

type Color = Vec3;

fn main() -> Result<(), std::io::Error> {
    let mut buffer = vec![];
    let progress = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                (i as f32) / (IMAGE_WIDTH - 1) as f32,
                (j as f32) / (IMAGE_HEIGHT - 1) as f32,
                0.0
            );

            buffer.push(color);
            progress.inc(1);
        }
    }

    let buffer: Vec<u8> = buffer.iter()
        .flat_map(to_rbg)
        .collect();

    let file = File::create("traced.png").unwrap();
    let encoder = PngEncoder::new(file);
    encoder.write_image(&buffer, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, ExtendedColorType::Rgb8).unwrap();

    progress.finish();

    Ok(())
}

fn to_rbg(color: &Color) -> [u8; 3] {
    let r = (255.999 * color.x) as u8;
    let g = (255.999 * color.y) as u8;
    let b = (255.999 * color.z) as u8;

    [r, g, b]
}