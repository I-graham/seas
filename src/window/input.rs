use super::*;

pub struct Input {
	pub size: (u32, u32),
	pub scroll: f32,
	pub mouse_pos: (f32, f32),
	pub left_mouse: ui::MouseState,
	pub right_mouse: ui::MouseState,
	pub keymap: fnv::FnvHashMap<winit::event::VirtualKeyCode, bool>,
}

impl Input {
	pub fn mouse_button(&mut self, button: &winit::event::MouseButton, down: bool) {
		use winit::event::MouseButton::{Left, Right};
		match button {
			Left => self.left_mouse.update(down),
			Right => self.right_mouse.update(down),
			_ => (),
		}
	}

	pub fn update_mouse(&mut self) {
		self.left_mouse.update(self.left_mouse.is_down());
		self.right_mouse.update(self.right_mouse.is_down());
	}

	pub fn capture_mouse(&mut self, pos: &winit::dpi::PhysicalPosition<f64>) {
		self.mouse_pos = (
			2.0 * pos.x as f32 / self.size.0 as f32 - 1.0,
			-2.0 * pos.y as f32 / self.size.1 as f32 + 1.0,
		);
	}

	pub fn capture_key(&mut self, input: winit::event::KeyboardInput) {
		use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
		let KeyboardInput {
			virtual_keycode: key,
			state,
			..
		} = input;
		match key {
			Some(key) if (VirtualKeyCode::A..VirtualKeyCode::F12).contains(&key) => {
				self.keymap.insert(key, state == ElementState::Pressed);
			}
			_ => {}
		}
	}

	pub fn key(&self, key: winit::event::VirtualKeyCode) -> bool {
		*self.keymap.get(&key).unwrap_or(&false)
	}
}
