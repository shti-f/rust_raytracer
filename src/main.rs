extern crate image;
extern crate chrono;

use chrono::Utc;
use std::path::Path;

fn main() {
    let imgx = 800;
    let imgy = 800;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // Save the image as "*datetime*.png"
    let datetime = Utc::now().format("%Y_%m%d_%H%M%S").to_string();
    let path = Path::new("./imgs/").join(format!("{}{}", datetime, ".png"));
    imgbuf.save(path).unwrap();
}