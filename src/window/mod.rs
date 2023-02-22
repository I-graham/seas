mod reng;
mod types;
mod ui;

use types::*;
use ui::ClientTexture;

const START_WIN_SIZE: winit::dpi::PhysicalSize<f32> = winit::dpi::PhysicalSize {
    width: 400.0,
    height: 400.0,
};

pub struct WinApi {
    pub window: winit::window::Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub aspect: f32,
    pub scroll: f32,
    pub mouse_pos: (f32, f32),
    pub left_mouse: ui::MouseState,
    pub right_mouse: ui::MouseState,
    pub keymap: fnv::FnvHashMap<winit::event::VirtualKeyCode, bool>,
    renderer: reng::Renderer2D<Uniform, Instance2D>,
    view: Camera,
    texture_map: fnv::FnvHashMap<ClientTexture, Instance2D>,
}

impl WinApi {
    pub fn new(event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> Self {
        let window = winit::window::WindowBuilder::new()
            .with_min_inner_size(START_WIN_SIZE)
            .build(event_loop)
            .expect("unable to create window");

        let size = window.inner_size();

        let mut renderer = reng::Renderer2D::new(&window, 4);

        let (image, texture_map) = ClientTexture::load_textures();
        let texture = renderer.create_texture_from_image(&image);
        renderer.set_texture(&texture);

        Self {
            window,
            size,
            aspect: size.width as f32 / size.height as f32,
            scroll: 0.,
            mouse_pos: (0.0, 0.0),
            left_mouse: ui::MouseState::Up,
            right_mouse: ui::MouseState::Up,
            keymap: fnv::FnvHashMap::default(),
            renderer,
            view: Camera {
                pos: (0., 0.),
                scale: 1.,
            },
            texture_map,
        }
    }

    pub fn draw(&mut self) {
        self.renderer.clear(wgpu::Color::GREEN);
        self.renderer.draw(
            &Uniform {
                ortho: self.view.proj(self.aspect),
            },
            &[self.texture_map[&ClientTexture::ShipSheet]],
        );
    }

    pub fn submit(&mut self) {
        self.renderer.submit();
    }

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
            2.0 * pos.x as f32 / self.size.width as f32 - 1.0,
            -2.0 * pos.y as f32 / self.size.height as f32 + 1.0,
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

    pub fn resize(&mut self, dims: winit::dpi::PhysicalSize<u32>) {
        self.size = dims;
        self.aspect = dims.width as f32 / dims.height as f32;
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn key(&self, key: &winit::event::VirtualKeyCode) -> bool {
        *self.keymap.get(key).unwrap_or(&false)
    }
}
