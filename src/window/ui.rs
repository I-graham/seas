type Listener = Box<dyn FnMut(&mut Button)>;

pub struct Button {
    pub texture: super::types::Texture,
    pub tint: (f32, f32, f32, f32),
    pub coords: (f32, f32),
    pub size: (f32, f32),
    pub z_coord: f32,
    pub on_click: Option<Listener>,
}

impl Button {
    pub fn includes(&self, point: (f32, f32)) -> bool {
        super::utils::point_in_rect(point, self.size, self.coords)
    }
}

#[derive(PartialEq, Debug)]
pub enum MouseState {
    Up,
    Drag,
    Click,
    Release,
}

impl MouseState {
    pub fn update(&mut self, down: bool) {
        use MouseState::*;
        match *self {
            Up | Release if down => *self = Click,
            Click if down => *self = Drag,
            Drag | Click if !down => *self = Release,
            Release if !down => *self = Up,
            _ => (),
        }
    }

    pub fn is_down(&self) -> bool {
        use MouseState::*;
        match *self {
            Up | Release => false,
            Click | Drag => true,
        }
    }
}
