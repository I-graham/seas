use super::*;
use cgmath::*;

pub type Waypoint = Vector2<f32>;

pub struct Path {
	pub nodes: Vec<Waypoint>,
}

impl Path {
	pub const COLOR: GLvec4 = GLvec4(1., 0.25, 0.25, 1.);
	pub const NODE_SIZE: f32 = 3. * Self::THICKNESS;
	pub const THICKNESS: f32 = 15.;

	pub fn new(start: Waypoint) -> Self {
		Self { nodes: vec![start] }
	}

	pub fn move_first(&mut self, waypoint: Waypoint) {
		*self.nodes.first_mut().unwrap() = waypoint;
	}

	pub fn move_last(&mut self, waypoint: Waypoint) {
		*self.nodes.last_mut().unwrap() = waypoint;
	}

	pub fn add_waypoint(&mut self, point: Waypoint) {
		self.nodes.push(point);
	}

	pub fn finish(&mut self) {
		let _ = self.nodes.pop();
	}
}

impl GameObject for Path {
	type Scene = World;
	type Action = ();

	fn render(&self, win: &mut Window) {
		if self.nodes.is_empty() {
			return;
		}

		let mut start = self.nodes[0];
		for &node in &self.nodes[1..] {
			win.queue(Instance {
				color_tint: Self::COLOR,
				..win.external().line_instance(start, node, Self::THICKNESS)
			});
			start = node;
		}

		let node_instance = win.external().instance(Texture::Node);

		for &node in &self.nodes {
			win.queue(Instance {
				color_tint: Self::COLOR,
				position: node.into(),
				scale: GLvec2(Self::NODE_SIZE, Self::NODE_SIZE),
				..node_instance
			});
		}
	}
}
