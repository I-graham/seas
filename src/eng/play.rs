use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;

use super::*;
use crate::window::{Camera, TextureType};

pub trait Root: Sized + 'static {
	const TITLE: &'static str = "Game Window";

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
		//Generate image with inferno:
		//Windows:
		//type tracing.folded | inferno-flamegraph (OPTIONAL: --flamechart) > tracing-flamegraph.svg
		//Linux:
		//cat tracing.folded | inferno-flamegraph (OPTIONAL: --flamechart) > tracing-flamegraph.svg
		use tracing_chrome::ChromeLayerBuilder;
		use tracing_subscriber::prelude::*;

		let (flame_layer, _guard) = ChromeLayerBuilder::new().build();

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
					game.win.external_mut().capture_key(input);
				}

				WindowEvent::MouseWheel { delta, .. } => {
					use winit::dpi::PhysicalPosition;
					use winit::event::MouseScrollDelta::*;
					game.win.external_mut().scroll = match delta {
						LineDelta(_hor, ver) => ver,
						PixelDelta(PhysicalPosition { y, .. }) => y as f32,
					};
				}

				WindowEvent::CursorMoved { position, .. } => {
					let size = game.win.external().win_size;
					game.win.external_mut().capture_mouse(&position, size);
				}

				WindowEvent::MouseInput { button, state, .. } => game
					.win
					.external_mut()
					.mouse_button(&button, state == winit::event::ElementState::Pressed),

				WindowEvent::Destroyed => {
					tracing_guard.take();
					flow.set_exit()
				}

				_ => {}
			},

			Event::MainEventsCleared => {
				const FPS_FREQ: f64 = 5.;
				frame_counter += 1;
				let now = game.win.external().now;
				let time = now.duration_since(prev).as_secs_f64();
				if time > FPS_FREQ {
					println!("fps: {}", (frame_counter as f64 / FPS_FREQ) as i32);
					prev = now;
					frame_counter = 0;

					game.cleanup();
				}

				game.frame();
			}

			_ => {}
		}
	})
}
