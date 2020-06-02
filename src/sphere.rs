use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    pub positon: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64) -> Sphere {
        Sphere {
            positon: position,
            radius: radius,
        }
    }
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> bool {
        // 球の中心を原点とみなす
        let dir = ray.direction - self.positon;

        let a = dir.dot(dir);
        let b = 2.0 * ray.origin.dot(dir);
        let c = dir.dot(dir) - self.radius * self.radius;

        // 判別式D
        let d = b * b - 4.0 * a * c;

        return d >= 0.0;
    }
}
