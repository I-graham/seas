use cgmath::Vector2;

use super::*;
use crate::{game::*, window::Animation};

pub struct Wave {
    pos: Vector2<i32>,
    animation: Animation,
}

impl GameObject for Wave {
    fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Action> {
        if self.animation.finished(external.now) {
            Some(Action::Die)
        } else {
            None
        }
    }

    fn instance(&self, external: &External) -> Option<Instance> {
        Some(Instance {
            position: self.pos.cast::<f32>().unwrap().into(),
            ..self.animation.frame(external)
        })
    }
}

impl Wave {
    pub fn maybe_spawn(external: &External) -> Option<Self> {
        const WAVE_DENSITY: f32 = 1. / 2_000.;
        let v = external.view_dims() / 2.;

        if probability(WAVE_DENSITY * external.delta * v.x * v.y) {
            let cam = external.camera.pos;
            let offset = v.map(|f| rand_in(-f, f));

            Some(Wave {
                pos: snap_to_grid(cam + offset, (16., 16.)),
                animation: Animation::new(Texture::Wave, 3., Animation::SIN, Some(1.0)),
            })
        } else {
            None
        }
    }
}