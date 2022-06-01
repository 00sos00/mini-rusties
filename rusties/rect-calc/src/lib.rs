#[derive(Debug)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self { x, y, radius }
    }
}

pub fn calc_rect(circles: &Vec<Circle>) -> (f32, f32, f32, f32) {
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
    let rect_center_x = rect_left_edge + rect_width / 2.0;
    let rect_center_y = rect_top_edge + rect_height / 2.0;

    (rect_width, rect_height, rect_center_x, rect_center_y)
}