use tracing::trace_span;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;

use super::*;
use crate::window::{Camera, TextureType};

pub trait Root: Sized + 'static {
	type Texture: TextureType;
	type Signal: SignalType;

	fn init(external: &External) -> Self;
	fn camera(&self, inputs: &External) -> Camera;

	fn plan(&self, _external: &External, _messenger: &Sender<Dispatch<Self::Signal>>);
	fn update(&mut self, _external: &External, _messenger: &Messenger<Self::Signal>);
	fn render(&self, win: &mut Window);
	fn cleanup(&mut self) {}
}

pub fn play<World: Root>() -> ! {
	let mut tracing_guard = if cfg!(feature = "profile") {
		use tracing_flame::FlameLayer;
		use tracing_subscriber::prelude::*;

		let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();

		tracing_subscriber::registry().with(flame_layer).init();

		Some(_guard)
	} else {
		None
	};

	let event_loop = EventLoop::new();
	let mut game = state::GameState::<World>::new(&event_loop);

	let mut prev = std::time::Instant::now();
	let mut frame_counter = 0;

	event_loop.run(move |event, _, flow| {
		flow.set_poll();
		match event {
			Event::WindowEvent { event, window_id } if window_id == game.win.id() => match event {
				WindowEvent::CloseRequested => {
					flow.set_exit();
				}

				WindowEvent::Resized(dims) if dims.height != 0 && dims.width != 0 => {
					game.win.resize(dims);
				}

				WindowEvent::KeyboardInput { input, .. } => {
					game.win.inputs_mut().capture_key(input);
				}

				WindowEvent::MouseWheel { delta, .. } => {
					use winit::dpi::PhysicalPosition;
					use winit::event::MouseScrollDelta::*;
					game.win.inputs_mut().scroll = match delta {
						LineDelta(_hor, ver) => ver,
						PixelDelta(PhysicalPosition { y, .. }) => y as f32,
					};
				}

				WindowEvent::CursorMoved { position, .. } => {
					let size = game.win.inputs().win_size;
					game.win.inputs_mut().capture_mouse(&position, size);
				}

				WindowEvent::MouseInput { button, state, .. } => game
					.win
					.inputs_mut()
					.mouse_button(&button, state == winit::event::ElementState::Pressed),

				WindowEvent::Destroyed => {
					tracing_guard.take();
					flow.set_exit()
				}

				_ => {}
			},

			Event::MainEventsCleared => {
				{
					const FPS_FREQ: f64 = 5.;
					frame_counter += 1;
					let now = game.win.inputs().now;
					let time = now.duration_since(prev).as_secs_f64();
					if time > FPS_FREQ {
						println!("fps: {}", (frame_counter as f64 / FPS_FREQ) as i32);
						prev = now;
						frame_counter = 0;
						game.cleanup();
					}
				}

				game.step();
				game.draw();
				{
					let span = trace_span!("Presenting.");
					let _gaurd = span.enter();
					game.win.submit();
				}
			}

			_ => {}
		}
	})
}
