use super::*;

pub struct Display<T: TextureType> {
	animation: Animation<T>,
	lcl_rect: UIRect,
	gbl_rect: UIRect,
}

impl<T: TextureType> Display<T> {
	pub fn new(animation: Animation<T>) -> Self {
		Self {
			animation,
			lcl_rect: Default::default(),
			gbl_rect: Default::default(),
		}
	}
}

impl<T: TextureType> GameObject for Display<T> {
	type Scene = Parent;
	type Action = UIAction;

	fn instance(&self, external: &External) -> Option<Instance> {
		Instance {
			screen_relative: GLbool::True,
			position: self.gbl_rect.offset.into(),
			scale: self.gbl_rect.size.into(),
			..self.animation.frame(external)
		}
		.scale2(1. / external.aspect(), 1.)
		.into()
	}
}

impl<T: TextureType> UIElement for Display<T> {
	fn rect(&self) -> &UIRect {
		&self.lcl_rect
	}

	fn rect_mut(&mut self) -> &mut UIRect {
		&mut self.lcl_rect
	}

	fn propagate_global(&mut self, parent: &UIRect) {
		self.gbl_rect = self.lcl_rect.globalize(parent);
	}
}
