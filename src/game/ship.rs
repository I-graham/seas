use super::ui::Button;
use crate::game::Action;
use crate::window::{Animation, PlayMode, Context, Texture};
use std::time::Instant;

use super::GameObject;

pub struct Ship {
    animation: Animation,
    button: Button<bool>,
}

impl Ship {
    pub fn new(context: &Context) -> Self {
        Self {
            animation: Animation::new(context, Texture::Water, 3.0, PlayMode::Forever),
            button: Button::new(
                context,
                false,
                Texture::ReadyButton,
                (-0.30, 0.30),
                (0.25, 0.25),
                Some(|c| *c = true),
                Some(|c| *c = false),
            ),
        }
    }
}

impl GameObject for Ship {
    fn update(&mut self, input: &crate::window::Input) -> Action {
        self.button.update(input);
        if self.button.state {
            self.animation.restart();
        }
        Action::Nothing
    }

    fn render(&mut self, context: &mut Context, now: Instant) {
        context.emit(self.animation.get_frame(now));
        self.button.render(context, now);
    }
}
