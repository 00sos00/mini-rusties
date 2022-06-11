use crate::State;
use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode},
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

        if state.input.key_pressed(VirtualKeyCode::Escape) {
            state.capture_mouse = !state.capture_mouse;
        }

        // TODO: on_key_input(input);
    }

    // Capture mouse
    // ~~~~~~~~~~~~~~~
    window.set_cursor_grab(state.capture_mouse).unwrap();
    window.set_cursor_visible(!state.capture_mouse);
    // ~~~~~~~~~~~~~~~

    match event {
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta },
            ..
        } => {
            state.mouse_offset = (delta.0 as f32, delta.1 as f32);
        }
        Event::RedrawRequested(_) => {
            state.update();

            if let Err(e) = state.render() {
                eprintln!("{:?}", e);
            }

            state.dt = state.current_time.elapsed().as_secs_f32();
            state.current_time = std::time::Instant::now();
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    }
}
