use super::*;

#[derive(Default)]
pub struct Parent {
	children: Vec<Box<dyn UIElement>>,
	rect: UIRect,
}

impl Parent {
	pub fn screen(external: &External) -> Self {
		Self {
			children: vec![],
			rect: UIRect::screen(external),
		}
	}

	pub fn with(mut self, ui: impl UIElement + 'static) -> Self {
		self.children.push(Box::new(ui));
		self
	}
}

impl GameObject for Parent {
	type Scene = Parent;
	type Action = UIAction;

	fn plan(&self, scene: &Self::Scene, external: &External, messenger: &Sender<Parent::Signal>) {
		for child in &self.children {
			child.plan(scene, external, messenger);
		}
	}

	fn update(&mut self, external: &External, messenger: &Parent::Signal) -> Option<Self::Action> {
		for child in &mut self.children {
			child.update(external, messenger);
		}
		None
	}

	fn render(&self, win: &mut Window) {
		for child in &self.children {
			child.render(win);
		}
	}

	fn cleanup(&mut self) {
		for child in &mut self.children {
			child.cleanup();
		}
	}
}

impl UIElement for Parent {
	fn rect(&self) -> &UIRect {
		&self.rect
	}

	fn rect_mut(&mut self) -> &mut UIRect {
		&mut self.rect
	}

	fn propagate_global(&mut self, parent: &UIRect) {
		for child in &mut self.children {
			child.propagate_global(&self.rect.globalize(parent));
		}
	}
}
