pub mod messenger;
pub mod play;
pub mod state;
pub mod utils;

pub use messenger::*;
pub use play::*;
pub use std::sync::mpsc::Sender;
pub use utils::*;

use crate::window::{External, Instance, Window};

type Signal<S> = <<S as GameObject>::Scene as Root>::Signal;

pub trait GameObject {
	type Scene: Root;
	type Action;

	fn plan(
		&self,
		_scene: &Self::Scene,
		_external: &External,
		_messenger: &Sender<Dispatch<Signal<Self>>>,
	) {
	}

	fn update(
		&mut self,
		_external: &External,
		_messenger: &Messenger<Signal<Self>>,
	) -> Option<Self::Action> {
		None
	}

	//If object renders a single instance, this can be implemented instea
	//of GameObject::render
	fn instance(&self, _external: &External) -> Option<Instance> {
		None
	}

	fn render(&self, win: &mut Window) {
		if let Some(inst) = self.instance(win.inputs()) {
			win.clip(inst);
		}
	}

	//not ever guaranteed to be called. Usefor for occasional but not
	//mandatory cleanup to improve performance or release unused resources.
	fn cleanup(&mut self) {}
}
