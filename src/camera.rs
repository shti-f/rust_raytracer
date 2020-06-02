use super::vec3::Vec3;

/// ideal pinhole camera
pub struct Camera {
    /// position: センサーの中心座標（ベクトルで表現）
    pub sensor_position: Vec3,
    /// sensor_size
    pub sensor_size: [usize; 2],
    /// ピンホールの位置座標（ベクトルで表現）
    pub aperture_position: Vec3,
    /// up direction: 右手方向（単位ベクトル）
    pub up: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            sensor_position: Vec3::new(0.0, 0.0, -100.0),
            sensor_size: [800, 800],
            aperture_position: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(-1.0, 0.0, 0.0),
        }
    }
}
