use crate::State;
use winit::{
    event::Event,
    event_loop::ControlFlow,
    window::Window,
};

pub fn handle<T>(
    state: &mut State,
    window: &Window,
    event: Event<T>,
    control_flow: &mut ControlFlow,
) {
    if state.input.update(&event) {
        if state.input.quit() {
            *control_flow = ControlFlow::Exit;
        }

        if let Some(size) = state.input.window_resized() {
            state.resize(size);
        }

        // TODO: on_key_input(input);
    }

    match event {
        Event::RedrawRequested(_) => {
            state.update();

            if let Err(e) = state.render() {
                eprintln!("{:?}", e);
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    }
}
