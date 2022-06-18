mod camera;
mod cube;
mod event_handler;
mod state;
mod texture;
mod transform;
mod vertex;

/* use env_logger::Builder;
use log::LevelFilter; */
use state::State;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    // Builder::new().filter_level(LevelFilter::Info).init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("learning-wgpu")
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        event_handler::handle(&mut state, &window, event, control_flow);
    });
}
