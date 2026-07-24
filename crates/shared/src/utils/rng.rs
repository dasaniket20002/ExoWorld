pub fn random_f32(min: f32, max: f32) -> f32 {
    (fastrand::f32() * (max - min + 1.0)) + min
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    (fastrand::f64() * (max - min + 1.0)) + min
}
