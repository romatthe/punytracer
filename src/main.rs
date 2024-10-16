use std::fs::File;

use image::{ExtendedColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use indicatif::ProgressBar;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() -> Result<(), std::io::Error> {
    let mut buffer = [0u8; IMAGE_WIDTH * IMAGE_HEIGHT * 3];
    let progress = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let g = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            buffer[(IMAGE_HEIGHT * j * 3) + (i * 3)] = ir;
            buffer[(IMAGE_HEIGHT * j * 3) + (i * 3) + 1] = ig;
            buffer[(IMAGE_HEIGHT * j * 3) + (i * 3) + 2] = ib;

            progress.inc(1);
        }
    }

    let file = File::create("traced.png").unwrap();
    let encoder = PngEncoder::new(file);
    encoder.write_image(&buffer, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, ExtendedColorType::Rgb8).unwrap();

    progress.finish();

    Ok(())
}