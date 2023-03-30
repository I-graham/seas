use super::*;

pub trait Automaton {
	type State: Copy + Eq;
	
	fn state(&self) -> Self::State;
	fn state_mut(&mut self) -> &mut Self::State;
	fn enter_from(&mut self, _old : Self::State) {}
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


	fn plan(&self, _world: &World, _external: &External, _input: &Input) {}

	fn update(&mut self, _external: &External) -> Option<Action> {
		None
	}

	fn render(&self, _context: &External, _out: &mut Vec<Instance>) {}
}

impl<T: Automaton> GameObject for T {
	fn plan(&self, world: &World, external: &External, input: &Input) {
		Automaton::plan(self, world, external, input)
	}

	fn update(&mut self, external: &External) -> Option<Action> {
		let old = self.state();
		let new = self.next_state(external);

		if new != old {
			self.exit_to(new);
			*self.state_mut() = new;
			self.enter_from(old);
		}

		Automaton::update(self, external)
	}

	fn render(&self, context: &External, out: &mut Vec<Instance>) {
		Automaton::render(self, context, out)
	}
}
