use std::fs::File;

use glam::DVec3;
use image::{ExtendedColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use indicatif::ProgressBar;
use crate::ray::Ray;

mod ray;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = match (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64 {
    n if n < 1 => 1 as usize,
    n          => n as usize,
};

// Camera
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH / IMAGE_HEIGHT) as f64;

pub type Vec3 = DVec3;
pub type Color = DVec3;
pub type Point3 = DVec3;

fn main() -> Result<(), std::io::Error> {
    let mut buffer = vec![];
    let progress = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);

    let camera_center= Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(VIEWPORT_WIDTH as f64, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT as f64, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel_zero_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel_zero_location + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);

            let color = ray_color(&ray);

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

fn ray_color(ray: &Ray) -> Color {
    // TODO: Is normalized same as Unit vector?
    // inline vec3 unit_vector(const vec3& v) {
    //     return v / v.length();
    // }
    // let unit_direction = ray.direction().normalize();
    let unit_direction = ray.direction() / ray.direction().length();
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + Color::new(0.5, 0.7, 1.0) * a
}