mod window;

use winit::event::{Event, WindowEvent};

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let mut win = window::WinApi::new(&event_loop);

    event_loop.run(move |event, _, flow| {
        flow.set_poll();
        match event {
            Event::WindowEvent { event, window_id } if window_id == win.id() => match event {
                WindowEvent::CloseRequested => {
                    flow.set_exit();
                }

                WindowEvent::Resized(dims) if dims.height != 0 && dims.width != 0 => {
                    win.resize(dims);
                }

                WindowEvent::KeyboardInput { input, .. } => win.capture_key(input),

                WindowEvent::MouseWheel { delta, .. } => {
                    use winit::dpi::PhysicalPosition;
                    use winit::event::MouseScrollDelta::*;
                    win.scroll = match delta {
                        LineDelta(_hor, ver) => ver,
                        PixelDelta(PhysicalPosition { y, .. }) => y as f32,
                    };
                }

                WindowEvent::CursorMoved { position, .. } => win.capture_mouse(&position),

                WindowEvent::MouseInput { button, state, .. } => {
                    win.mouse_button(&button, state == winit::event::ElementState::Pressed)
                }

                _ => {}
            },

            Event::MainEventsCleared => {
                win.window.request_redraw();
            }

            Event::RedrawRequested(id) if id == win.id() => {
                win.draw();
                win.submit();
            }

            _ => {}
        }
    });
}
