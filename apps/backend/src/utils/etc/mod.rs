pub mod par_write_ptr;

pub fn random_f32(min: f32, max: f32) -> f32 {
    min + fastrand::f32() * (max - min)
}

// pub fn random_f64(min: f64, max: f64) -> f64 {
//     min + fastrand::f64() * (max - min)
// }
