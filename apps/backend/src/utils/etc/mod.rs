pub fn random_f32(min: f32, max: f32) -> f32 {
    min + fastrand::f32() * (max - min)
}

// pub fn random_f64(min: f64, max: f64) -> f64 {
//     min + fastrand::f64() * (max - min)
// }

pub fn fast_inv_sqrt(x: f32) -> f32 {
    let x2 = x * 0.5;
    let mut y = x;

    // Reinterpret bits as i32, apply magic number, and reinterpret back
    let mut i = y.to_bits() as i32;
    i = 0x5f3759df - (i >> 1);
    y = f32::from_bits(i as u32);

    // One iteration of Newton's method for refinement
    y * (1.5 - x2 * y * y)
}
