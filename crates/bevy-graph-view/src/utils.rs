use rand::Rng;
use std::f64::consts::PI;

pub fn random_point_on_circle_surface(radius: f32) -> forceatlas2::Vec2<f32> {
    let mut rng = rand::thread_rng();
    let theta = rng.gen::<f32>() * 2.0 * PI as f32;

    let x = radius * theta.cos();
    let y = radius * theta.sin();

    forceatlas2::Vec2::new(x, y)
}
