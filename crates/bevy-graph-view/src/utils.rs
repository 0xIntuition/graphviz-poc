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

pub fn random_point_on_circle_surface(radius: f32) -> forceatlas2::Vec2<f32> {
    let mut rng = rand::thread_rng();
    let theta = rng.gen::<f32>() * 2.0 * PI as f32;

    let x = radius * theta.cos();
    let y = radius * theta.sin();

    forceatlas2::Vec2::new(x, y)
}

pub fn normalize(vec: &mut [f32]) {
    let min = vec.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = vec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    if max - min != 0.0 {
        vec.iter_mut()
            .for_each(|x| *x = 1.0 + (*x - min) / (max - min));
    }
}
