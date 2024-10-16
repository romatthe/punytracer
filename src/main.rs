const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() -> Result<(), std::io::Error> {
    let mut buffer = [0u8; IMAGE_WIDTH * IMAGE_HEIGHT * 3];

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let g = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir as u8, ig as u8, ib as u8);
        }
    }

    // image::save_buffer("traced.png", &buffer, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, image::ExtendedColorType::Rgb8).unwrap();
    Ok(())
}