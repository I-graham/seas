use super::*;

pub trait StateMachine {
	type State: Copy + Eq;
	
	fn state(&self) -> Self::State;
	fn state_mut(&mut self) -> &mut Self::State;
	fn enter(&mut self, _new: Self::State) {}
	fn exit(&mut self, _old: Self::State) {}
	fn next_state(&self, external: &External) -> Self::State;
	
	fn by_table(&self, probability_table: &[(Self::State, f32)]) -> Self::State {
		
		let mut rng = random();
		for &(state, prob) in probability_table {
			if rng < prob {
				return state;
			}
			rng -= prob;
		}

		self.state()
	}


	fn plan(&self, _world: &World, _external: &External, _input: &Input) {}

	fn update(&mut self, _external: &External) -> Option<Action> {
		None
	}

	fn render(&self, _context: &External, _out: &mut Vec<Instance>) {}
}

impl<T: StateMachine> GameObject for T {
	fn plan(&self, world: &World, external: &External, input: &Input) {
		StateMachine::plan(self, world, external, input)
	}

	fn update(&mut self, external: &External) -> Option<Action> {
		let old = self.state();
		let new = self.next_state(external);

		if new != old {
			self.exit(old);
			*self.state_mut() = new;
			self.enter(new);
		}

		StateMachine::update(self, external)
	}

	fn render(&self, context: &External, out: &mut Vec<Instance>) {
		StateMachine::render(self, context, out)
	}
}
