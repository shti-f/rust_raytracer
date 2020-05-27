use image;
use std::path::PathBuf;
use super::vec3::Vec3;

pub struct Img {
    data: Vec<Vec<Vec3>>,
    width: usize,
    height: usize,
}

impl Img {
    pub fn new(width: usize, height: usize) -> Img {
        Img {
            data: vec![vec![Vec3::new(0.0, 0.0, 0.0); width]; height],
            width: width,
            height: height,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, v: Vec3) {
        self.data[y][x] = v;
    }

    pub fn get(&self, x: usize, y: usize) -> Vec3 {
        self.data[y][x]
    }

    pub fn update_each<F>(&mut self, f: F)
    where F: Fn(usize, usize) -> Vec3
    {
        for x in 0..self.width {
            for y in 0..self.height {
                self.data[y][x] = f(x, y)
            }
        }
    }

    pub fn save_img(&self, path: &PathBuf) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let output_pixel = self.get(x as usize, y as usize);
            *pixel = image::Rgb( [output_pixel.x as u8, output_pixel.y as u8, output_pixel.z as u8] );
        }
        imgbuf.save(path).unwrap();
    }
}

