use super::*;
use cgmath::*;

type Waypoint = Vector2<f32>;

pub struct Route {
	nodes: Vec<Waypoint>,
}

impl Route {
	pub const COLOR: (f32, f32, f32, f32) = (1., 0.25, 0.25, 1.);
	pub const NODE_SIZE: f32 = 2. * Self::THICKNESS;
	pub const THICKNESS: f32 = 20.;

	pub fn new() -> Self {
		Self { nodes: vec![] }
	}

	pub fn add_waypoint(&mut self, point: Waypoint) {
		self.nodes.push(point);
	}
}

impl GameObject for Route {
	type Scene = World;
	type Action = ();

	fn render(&self, win: &mut Window) {
		if self.nodes.is_empty() {
			return;
		}

		let node_instance = win.external().instance(Texture::Node);

		for &node in &self.nodes {
			win.queue(Instance {
				color_tint: Self::COLOR.into(),
				position: node.into(),
				scale: (Self::NODE_SIZE * vec2(1., 1.)).into(),
				..node_instance
			});
		}

		let mut start = self.nodes[0];
		for &node in &self.nodes[1..] {
			win.queue(Instance {
				color_tint: Self::COLOR.into(),
				..win.external().line_instance(start, node, Self::THICKNESS)
			});
			start = node;
		}
	}
}
