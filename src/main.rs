use chrono::Utc;
use std::path::Path;

use renderer::vec3;
use renderer::img;

fn main() {
    let mut output = img::Img::new(800, 800);

    output.update_each(|x, y| {
        let r = 0.3 * x as f64;
        let b = 0.3 * y as f64;
        vec3::Vec3::new(r, 0.0, b)
    });

    // Save the image as "*datetime*.png"
    let datetime = Utc::now().format("%Y_%m%d_%H%M%S").to_string();
    let path = Path::new("./imgs/").join(format!("{}{}", datetime, ".png"));
    output.save_img(&path);
}