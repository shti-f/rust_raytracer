use chrono::Local;
use std::path::PathBuf;

use renderer::img;
use renderer::vec3::Vec3;

fn main() {
    let mut output = img::Img::new(800, 800);

    fn pixel_updater(x: usize, y: usize) -> Vec3 {
        let r = 0.3 * x as f64;
        let b = 0.3 * y as f64;
        Vec3::new(r, 0.0, b)
    }

    output.parallel_update_each(pixel_updater);

    // Save the image as "*datetime*.png"
    let datetime = Local::now().format("%Y_%m%d_%H%M%S").to_string();
    let mut path = PathBuf::new();
    path.push("./imgs/");
    path.push(format!("{}{}", datetime, ".png"));
    output.save_img(&path);
}
