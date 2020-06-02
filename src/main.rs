use chrono::Local;
use std::path::PathBuf;

use renderer::camera::Camera;
use renderer::img;
use renderer::ray::Ray;
use renderer::sphere::Sphere;
use renderer::vec3::Vec3;

fn main() {
    let cam = Camera::new();
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, 150.0), 100.0);
    let scene = vec![sphere1];

    let imagex = cam.sensor_size[0];
    let imagey = cam.sensor_size[1];
    let mut output = img::Img::new(imagex, imagey);

    fn pixel_updater(x: usize, y: usize, cam: &Camera, scene: &Vec<Sphere>) -> Vec3 {
        let imagex = cam.sensor_size[0];
        let imagey = cam.sensor_size[1];
        let originx = x as f64 - imagex as f64 / 2.0;
        let originy = y as f64 - imagey as f64 / 2.0;
        let origin = Vec3::new(originx, originy, cam.sensor_position.z);
        let direction = cam.aperture_position - origin;
        let ray = Ray::new(origin, direction);

        let result = scene.iter().any(|sphere| sphere.intersect(&ray));
        match result {
            true => Vec3::new(200.0, 0.0, 0.0),
            false => Vec3::new(0.0, 0.0, 0.0),
        }
    }

    output.parallel_update_each_with_camera_scene(pixel_updater, &cam, &scene);

    // Save the image as "*datetime*.png"
    let datetime = Local::now().format("%Y_%m%d_%H%M%S").to_string();
    let mut path = PathBuf::new();
    path.push("./imgs/");
    path.push(format!("{}{}", datetime, ".png"));
    output.save_img(&path);
}
