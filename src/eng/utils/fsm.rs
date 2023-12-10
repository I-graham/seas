use super::*;

type Signal<S> = <<S as Automaton>::FsmScene as Root>::Signal;

pub trait Automaton {
	type FsmScene: Root;
	type FsmAction;
	type State: Copy + Eq;

	fn state(&self) -> Self::State;
	fn state_mut(&mut self) -> &mut Self::State;
	fn enter_from(&mut self, _old: Self::State) {}
	fn exit_to(&mut self, _new: Self::State) {}
	fn next_state(&self, external: &External) -> Self::State;

	fn by_probability(&self, probability_table: &[(Self::State, f32)]) -> Self::State {
		let mut rng = random();
		for &(state, prob) in probability_table {
			if rng < prob {
				return state;
			}
			rng -= prob;
		}
		self.state()
	}

	fn fsm_plan(
		&self,
		_scene: &Self::FsmScene,
		_external: &External,
		_messenger: &Sender<Dispatch<Signal<Self>>>,
	) {
	}

	fn fsm_update(
		&mut self,
		_external: &External,
		_messenger: &Messenger<Signal<Self>>,
	) -> Option<Self::FsmAction> {
		None
	}

	fn fsm_instance(&self, _external: &External) -> Option<Instance> {
		None
	}

	fn fsm_render(&self, win: &mut Window) {
		if let Some(inst) = self.fsm_instance(win.external()) {
			win.queue(inst);
		}
	}

	fn fsm_cleanup(&mut self) {}
}

impl<T: Automaton> GameObject for T {
	type Scene = T::FsmScene;
	type Action = T::FsmAction;

	fn plan(
		&self,
		scene: &Self::Scene,
		external: &External,
		messenger: &Sender<Dispatch<Signal<Self>>>,
	) {
		self.fsm_plan(scene, external, messenger)
	}

	fn update(
		&mut self,
		external: &External,
		messenger: &Messenger<Signal<Self>>,
	) -> Option<Self::Action> {
		let old = self.state();
		let new = self.next_state(external);

		if new != old {
			self.exit_to(new);
			*self.state_mut() = new;
			self.enter_from(old);
		}

		self.fsm_update(external, messenger)
	}

	fn render(&self, win: &mut Window) {
		self.fsm_render(win)
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		self.fsm_instance(external)
	}

	fn cleanup(&mut self) {
		self.fsm_cleanup()
	}
}
