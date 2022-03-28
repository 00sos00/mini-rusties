use core::ops::Range;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn with_random_position(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(x_range);
        let y = rng.gen_range(y_range);

        Self { x, y }
    }

    pub fn distance_from(&self, other_point: &Point) -> f32 {
        ((other_point.x - self.x).powf(2.0) + (other_point.y - self.y).powf(2.0)).sqrt()
    }
}
