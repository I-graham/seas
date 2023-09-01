mod boats;
mod map;

use boats::*;
use map::*;

pub use super::*;
use crate::window::External;

pub struct World {
    pub map: Map,
    pub raft: Raft,
}

const MAP_SIZE: u32 = 500 * 32;
impl World {
    pub fn new() -> Self {
        Self {
            map: Map::new(MAP_SIZE),
            raft: Raft::new(),
        }
    }
}

impl GameObject for World {
    fn plan(&self, world: &World, external: &External, input: &Input, messenger: &mut Messenger) {
        self.map.plan(world, external, input, messenger);
        self.raft.plan(world, external, input, messenger);
    }

    fn update(&mut self, external: &External, messenger: &Messenger) -> Option<super::Action> {
        self.map.update(external, messenger);
        self.raft.update(external, messenger);
        None
    }

    fn render(&self, external: &External, out: &mut Vec<Instance>) {
        self.map.render(external, out);
        self.raft.render(external, out);
    }
}
