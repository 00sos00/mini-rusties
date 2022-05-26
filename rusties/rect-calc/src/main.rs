use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

fn calc_rect(circles: &Vec<Circle>) -> (f32, f32, f32, f32) {
    let mut rect_left_edge = 0.0;
    let mut rect_right_edge = 0.0;
    let mut rect_top_edge = 0.0;
    let mut rect_bottom_edge = 0.0;

    for c in circles {
        let min_x = c.x - c.radius;
        let max_x = c.x + c.radius;
        let max_y = c.y + c.radius;
        let min_y = c.y - c.radius;

        if min_x < rect_left_edge || rect_left_edge == 0.0 {
            rect_left_edge = min_x;
        }
        if max_x > rect_right_edge || rect_right_edge == 0.0 {
            rect_right_edge = max_x;
        }
        if max_y > rect_top_edge || rect_top_edge == 0.0 {
            rect_top_edge = max_y;
        }
        if min_y < rect_bottom_edge || rect_bottom_edge == 0.0 {
            rect_bottom_edge = min_y;
        }
    }

    let rect_width = rect_right_edge - rect_left_edge;
    let rect_height = rect_top_edge - rect_bottom_edge;
    let rect_center_x = (rect_left_edge + rect_right_edge) / 2.0 - (rect_width / 2.0);
    let rect_center_y = (rect_top_edge + rect_bottom_edge) / 2.0 - (rect_height / 2.0);

    (rect_width, rect_height, rect_center_x, rect_center_y)
}

fn main() {
    let mut rng = thread_rng();

    let circles_amount = 25;
    let min_circle_radius = 25.0;
    let max_circle_radius = 50.0;
    let x_spawn_bounds = (100.0, 500.0);
    let y_spawn_bounds = (50.0, 400.0);

    let mut circles = vec![];

    for _ in 0..circles_amount {
        let rx = rng.gen_range(x_spawn_bounds.0..x_spawn_bounds.1);
        let ry = rng.gen_range(y_spawn_bounds.0..y_spawn_bounds.1);
        let rs = rng.gen_range(min_circle_radius..max_circle_radius);

        circles.push(Circle {
            x: rx,
            y: ry,
            radius: rs,
        });
    }

    let (rect_width, rect_height, rect_center_x, rect_center_y) = calc_rect(&circles);

    println!("Width: {rect_width}\nHeight: {rect_height}\nCenter_X: {rect_center_x}\nCenter_Y: {rect_center_y}");
}
