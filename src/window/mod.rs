pub mod glsl;
mod loader;
mod reng;
mod types;

pub use glsl::*;
use std::time::Instant;

pub use types::*;

const START_WIN_SIZE: winit::dpi::PhysicalSize<f32> = winit::dpi::PhysicalSize {
	width: 800.0,
	height: 600.0,
};

#[derive(PartialEq, Eq)]
enum DrawKind {
	Cached,
	Uncached,
}

pub struct Window {
	window: winit::window::Window,
	inputs: External,
	output: Vec<Instance>,
	draw_kind: DrawKind,
	renderer: reng::Renderer<glsl::Uniform, Instance>,
}

impl Window {
	pub fn new<Texture: TextureType>(
		event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
	) -> Self {
		let window = winit::window::WindowBuilder::new()
			.with_min_inner_size(START_WIN_SIZE)
			.build(event_loop)
			.expect("unable to create window");

		let size = window.inner_size();

		let mut renderer = reng::Renderer::new(&window, 4);

		let (image, texture_map) = loader::load_textures::<Texture>();
		let texture = renderer.create_texture_from_image(&image);
		renderer.set_texture(&texture);

		Self {
			window,
			renderer,
			inputs: External {
				scroll: 0.,
				mouse_pos: cgmath::vec2(0.0, 0.0),
				left_mouse: ButtonState::Up,
				right_mouse: ButtonState::Up,
				keymap: fnv::FnvHashMap::default(),
				texture_map,
				camera: Camera {
					pos: cgmath::vec2(0., 0.),
					scale: 600.,
				},
				win_size: (size.width, size.height),
				now: Instant::now(),
				delta: 0.,
			},
			draw_kind: DrawKind::Uncached,
			output: vec![],
		}
	}

	pub fn external_mut(&mut self) -> &mut External {
		&mut self.inputs
	}

	pub fn external(&self) -> &External {
		&self.inputs
	}

	pub fn clip(&mut self, instance: Instance) {
		//clip unseen instances
		if self.inputs.visible(instance) {
			self.output.push(instance);
		}
	}

	pub fn clear(&mut self) {
		//White for debugging purposes.
		self.renderer.clear(wgpu::Color::WHITE);
	}

	pub fn draw(&mut self) {
		self.renderer.set_uniform(glsl::Uniform {
			ortho: self.inputs.camera.proj(self.inputs.aspect()),
		});
		self.renderer.draw(&self.output);
		self.output.clear();
	}

	pub fn is_cached(&self, name: &'static str) -> bool {
		self.renderer.is_cached(name)
	}

	pub fn cache(&mut self, name: &'static str, instances: &[Instance]) {
		self.renderer.cache(name, instances)
	}


	pub fn draw_cached(&mut self, name: &'static str, pos: (f32, f32), scale: f32) {
		if self.draw_kind != DrawKind::Cached && !self.output.is_empty() {
			self.draw();
		}

		let shift: cgmath::Vector2<f32> = pos.into();
		self.renderer.set_uniform(glsl::Uniform {
			ortho: Camera {
				pos: self.inputs.camera.pos - shift,
				scale: self.inputs.camera.scale / scale,
			}
			.proj(self.inputs.aspect()),
		});

		self.renderer.draw_cached(name);
	}

	pub fn submit(&mut self) {
		self.renderer.submit();
	}

	pub fn resize(&mut self, dims: winit::dpi::PhysicalSize<u32>) {
		self.inputs.win_size = (dims.width, dims.height);
		self.renderer.resize(dims);
	}

	pub fn id(&self) -> winit::window::WindowId {
		self.window.id()
	}
}
