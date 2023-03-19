pub mod glsl;
mod input;
mod loader;
mod reng;
mod types;
pub mod ui;

pub use input::*;
pub use types::{Animation, PlayMode, Camera, Context, Instance, Texture, TextureMap};
pub use ui::*;
pub use glsl::*;
use std::time::Instant;

const START_WIN_SIZE: winit::dpi::PhysicalSize<f32> = winit::dpi::PhysicalSize {
    width: 400.0,
    height: 400.0,
};

pub struct WinApi {
    pub window: winit::window::Window,
    pub input: Input,
    pub context: Context,
    renderer: reng::Renderer<glsl::Uniform, Instance>,
    epoch: Instant, 
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
                scroll: 0.,
                mouse_pos: (0.0, 0.0),
                left_mouse: ui::MouseState::Up,
                right_mouse: ui::MouseState::Up,
                keymap: fnv::FnvHashMap::default(),
            },
            renderer,
            context: Context {
                texture_map,
                camera: Camera {
                    pos: (0., 0.),
                    scale: 1.,
                },
                size: (size.width, size.height),
                instances: vec![],
            },
            epoch: Instant::now(),
        }
    }

    pub fn clear(&mut self) {
        //Red for debugging purposes.
        self.context.instances.clear();
        self.renderer.clear(wgpu::Color::RED);
    }

    pub fn draw(&mut self) {
        self.renderer.set_uniform(glsl::Uniform {
            ortho: self.context.camera.proj(self.context.aspect()),
			time: Instant::now().duration_since(self.epoch).as_secs_f32(),
        });
        self.renderer.draw(&self.context.instances);
    }

    pub fn submit(&mut self) {
        self.renderer.submit();
    }

    pub fn resize(&mut self, dims: winit::dpi::PhysicalSize<u32>) {
        self.context.size = (dims.width, dims.height);
        self.renderer.resize(dims);
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }
}
