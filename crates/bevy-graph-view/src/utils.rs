use rand::Rng;
use std::f64::consts::PI;

pub fn random_point_on_sphere_surface(radius: f32) -> (f32, f32, f32) {
    let mut rng = rand::thread_rng();
    let theta = rng.gen::<f32>() * 2.0 * PI as f32;
    let phi = rng.gen::<f32>() * PI as f32;

    let x = radius * phi.sin() * theta.cos();
    let y = radius * phi.sin() * theta.sin();
    let z = radius * phi.cos();

    (x, y, z)
}
