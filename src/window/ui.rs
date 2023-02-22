use super::types::{GLvec2, GLvec4};
use super::{reng::*, types::Instance2D};

use fnv::FnvHashMap;
use std::hash::Hash;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

pub fn in_rect(point: (f32, f32), scale: (f32, f32), pos: (f32, f32)) -> bool {
    let (x1, x2) = (pos.0 - scale.0, pos.0 + scale.0);
    let (y1, y2) = (pos.1 - scale.1, pos.1 + scale.1);

    x1 < point.0 && point.0 < x2 && y1 < point.1 && point.1 < y2
}

#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ClientTexture {
    Troop,
}

impl ClientTexture {
    pub fn load_textures() -> (image::RgbaImage, FnvHashMap<ClientTexture, Instance2D>) {
        let mut map = FnvHashMap::default();

        let mut rgba_images = Self::iter()
            .map(|text_name| {
                let file_name = format!("assets/{}.png", <&'static str>::from(text_name));
                image::open(file_name).unwrap().into_rgba8()
            })
            .collect::<Vec<_>>();

        let img_size = |img: &image::RgbaImage| (img.height() * img.width()) as i32;

        let mut sorted_iter = Self::iter().enumerate().collect::<Vec<_>>();
        sorted_iter.sort_by_key(|(index, _text)| -img_size(&rgba_images[*index]));

        rgba_images.sort_by_key(|e| -img_size(e));

        let spritesheet = utils::create_spritesheet(rgba_images);

        let image_dims = spritesheet.0.dimensions();

        let pixel_to_text_coord = |(x, y)| {
            let norm_x = x as f32 / image_dims.0 as f32;
            let norm_y = y as f32 / image_dims.1 as f32;
            (norm_x, norm_y)
        };

        for (text, &(ul, lr)) in sorted_iter
            .iter()
            .map(|(_index, text)| text)
            .zip(&spritesheet.1)
        {
            let (ulx, uly) = pixel_to_text_coord(ul);
            let (lrx, lry) = pixel_to_text_coord(lr);

            let texture = GLvec4(ulx, uly, lrx, lry);

            map.insert(
                *text,
                Instance2D {
                    texture,
                    scale: GLvec2((lr.0 - ul.0) as f32 / (lr.1 - ul.1) as f32, 1.0),
                    ..Default::default()
                },
            );
        }

        (spritesheet.0, map)
    }
}

#[repr(i8)]
#[derive(EnumCount, Clone, Copy)]
pub enum UILayer {
    Foreground,
    Background,
}

impl UILayer {
    pub fn z_coord(&self) -> f32 {
        -(*self as i32 as f32 / Self::COUNT as f32)
    }
}

type Listener = Box<dyn FnMut(&mut Button)>;

pub struct Button {
    pub texture: ClientTexture,
    pub tint: (f32, f32, f32, f32),
    pub coords: (f32, f32),
    pub size: (f32, f32),
    pub z_coord: f32,
    pub on_click: Option<Listener>,
}

impl Button {
    pub fn includes(&self, point: (f32, f32)) -> bool {
        in_rect(point, self.size, self.coords)
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
