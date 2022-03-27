use crate::State;
use winit::{event::*, event_loop::ControlFlow, window::Window};

fn handle_key_press(
    _state: &mut State,
    _window: &Window,
    key: &VirtualKeyCode,
    _control_flow: &mut ControlFlow,
) {
    match key {
        VirtualKeyCode::A => {}
        VirtualKeyCode::F => {}
        _ => {}
    }
}

pub fn handle<T>(
    state: &mut State,
    window: &Window,
    event: Event<T>,
    control_flow: &mut ControlFlow,
) {
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => handle_key_press(state, window, key, control_flow),
            WindowEvent::Resized(physical_size) => state.resize(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    }
}
