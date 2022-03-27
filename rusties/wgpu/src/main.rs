mod event_handler;
mod state;
mod vertex;

use state::State;
use winit::{event_loop::EventLoop, window::WindowBuilder};
use vertex::Vertex;
use env_logger::Builder;
use log::LevelFilter;

const TRI_VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

const TRI_INDICES: &[u16] = &[
    0, 1, 2
];

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgpu-project")
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        event_handler::handle(&mut state, &window, event, control_flow);
    });
}
