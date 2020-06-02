use super::vec3::Vec3;
use image;
use rayon::prelude::*;
use std::path::PathBuf;

pub struct Img {
    data: Vec<Vec3>,
    width: usize,
    height: usize,
}

impl Img {
    pub fn new(width: usize, height: usize) -> Img {
        Img {
            data: vec![Vec3::new(0.0, 0.0, 0.0); width * height],
            width: width,
            height: height,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, v: Vec3) {
        let idx = y * self.width + x;
        self.data[idx] = v;
    }

    pub fn get(&self, x: usize, y: usize) -> Vec3 {
        let idx = y * self.width + x;
        self.data[idx]
    }

    pub fn update_each(&mut self, f: fn(x: usize, y: usize) -> Vec3) {
        for i in 0..(self.width * self.height) {
            let x = i / self.width;
            let y = i % self.height;
            self.set(x, y, f(x, y));
        }
    }

    pub fn parallel_update_each(&mut self, f: fn(x: usize, y: usize) -> Vec3) {
        let par_iter = self.data.par_iter().enumerate().map(|(i, _pixel)| {
            let x = i / self.width;
            let y = i % self.height;
            f(x, y)
        });
        self.data = par_iter.collect();
    }

    pub fn do_each(&self, f: fn(x: usize, y: usize)) {
        for i in 0..(self.width * self.height) {
            let x = i / self.width;
            let y = i % self.height;
            f(x, y);
        }
    }

    pub fn save_img(&self, path: &PathBuf) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let output_pixel = self.get(x as usize, y as usize);
            *pixel = image::Rgb([
                output_pixel.x as u8,
                output_pixel.y as u8,
                output_pixel.z as u8,
            ]);
        }
        imgbuf.save(path).unwrap();
    }
}
