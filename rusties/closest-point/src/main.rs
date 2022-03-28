mod point;

use point::Point;

fn main() {
    rand::thread_rng();

    let base_point = Point::new(0.0, 0.0);

    let num_points = 10;
    let mut points = vec![Point::new(0., 0.); num_points];
    points.fill_with(|| Point::with_random_position(0.0..1000.0, 0.0..1000.0));

    points.sort_by_key(|p| base_point.distance_from(p) as u32);

    let closest_point = points.get(0).unwrap();

    println!("{closest_point:?}");
}
