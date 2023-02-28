pub mod glsl;
pub mod ui;
mod input;
mod loader;
mod reng;
mod types;

pub use input::*;
pub use types::{Animation, Camera, Context, Instance, Texture, TextureMap};
pub use ui::*;

const START_WIN_SIZE: winit::dpi::PhysicalSize<f32> = winit::dpi::PhysicalSize {
	width: 400.0,
	height: 400.0,
};

pub struct WinApi {
	pub window: winit::window::Window,
	pub input: Input,
	pub context: Context,
	renderer: reng::Renderer<glsl::Uniform, Instance>,
}

impl WinApi {
	pub fn new(event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> Self {
		let window = winit::window::WindowBuilder::new()
			.with_min_inner_size(START_WIN_SIZE)
			.build(event_loop)
			.expect("unable to create window");

		let size = window.inner_size();

		let mut renderer = reng::Renderer::new(&window, 4);

		let (image, texture_map) = loader::load_textures();
		let texture = renderer.create_texture_from_image(&image);
		renderer.set_texture(&texture);

		Self {
			window,
			input: Input {
				size: (size.width, size.height),
				scroll: 0.,
				mouse_pos: (0.0, 0.0),
				left_mouse: ui::MouseState::Up,
				right_mouse: ui::MouseState::Up,
				keymap: fnv::FnvHashMap::default(),
			},
			renderer,
			context: Context {
				texture_map,
				aspect: START_WIN_SIZE.width / START_WIN_SIZE.height,
			},
		}
	}

	pub fn clear(&mut self) {
		//Green for debugging purposes.
		self.renderer.clear(wgpu::Color::GREEN);
	}

	pub fn draw(&mut self, camera: &Camera, instances: &[Instance]) {
		let (x, y) = self.input.size;
		self.renderer.set_uniform(glsl::Uniform {
			ortho: camera.proj(x as f32 / y as f32),
		});
		self.renderer.draw(instances);
	}

	pub fn submit(&mut self) {
		self.renderer.submit();
	}

	pub fn resize(&mut self, dims: winit::dpi::PhysicalSize<u32>) {
		self.input.size = (dims.width, dims.height);
		self.context.aspect = dims.width as f32 / dims.height as f32;
		self.renderer.resize(dims);
	}

	pub fn id(&self) -> winit::window::WindowId {
		self.window.id()
	}
}
