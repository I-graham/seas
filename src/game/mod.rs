mod ship;
mod state;
mod ui;
mod world;
mod map;

use crate::window::{Context, Input};
use std::time::Instant;
use winit::event_loop::EventLoop;

pub enum Action {
	Nothing,
}

pub trait GameObject {
	fn update(&mut self, _input: &Input) -> Action {
		Action::Nothing
	}

	fn render(&mut self, _context: &mut Context, _now: Instant) {}
}

pub fn play() -> ! {
	use winit::event::{Event, WindowEvent};

	let event_loop = EventLoop::new();
	let mut game = state::GameState::new(&event_loop);

	#[cfg(debug_assertions)]
	let mut prev = std::time::Instant::now();
	#[cfg(debug_assertions)]
	let mut count = 0;
	
	event_loop.run(move |event, _, flow| {
		flow.set_poll();
		match event {
			Event::WindowEvent { event, window_id } if window_id == game.api.id() => match event {
				WindowEvent::CloseRequested => {
					flow.set_exit();
				}

				WindowEvent::Resized(dims) if dims.height != 0 && dims.width != 0 => {
					game.api.resize(dims);
				}

				WindowEvent::KeyboardInput { input, .. } => game.api.input.capture_key(input),

				WindowEvent::MouseWheel { delta, .. } => {
					use winit::dpi::PhysicalPosition;
					use winit::event::MouseScrollDelta::*;
					game.api.input.scroll = match delta {
						LineDelta(_hor, ver) => ver,
						PixelDelta(PhysicalPosition { y, .. }) => y as f32,
					};
				}

				WindowEvent::CursorMoved { position, .. } => {
					game.api
						.input
						.capture_mouse(&position, game.api.context.size);
				}

				WindowEvent::MouseInput { button, state, .. } => game
					.api
					.input
					.mouse_button(&button, state == winit::event::ElementState::Pressed),

				_ => {}
			},

			Event::MainEventsCleared => {

				#[cfg(debug_assertions)]
				{
					count += 1;
					let now = std::time::Instant::now();
					let time = now.duration_since(prev).as_secs_f64();
					if time > 1. {
						dbg!(count as f64 / time);
						prev = now;
						count = 0;
					}
				}
				

				game.update();
				game.draw();
				game.api.submit();
			}

			_ => {}
		}
	})
}
