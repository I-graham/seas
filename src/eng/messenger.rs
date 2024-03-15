use cgmath::*;
use std::sync::mpsc;
use std::time::*;

use super::{Grid, Griddable};

pub trait SignalType {
	type SignalKinds: Copy + Into<usize> + PartialEq;
	const COUNT: usize;

	fn kind(&self) -> Self::SignalKinds;
}

pub struct Messenger<S: SignalType> {
	now: Instant,
	global: Buckets<S>,
	locals: Grid<(Instant, Dispatch<S>)>,
	sender: mpsc::Sender<Dispatch<S>>,
	receiver: mpsc::Receiver<Dispatch<S>>,
}

unsafe impl<S: SignalType> Send for Messenger<S> {}
unsafe impl<S: SignalType> Sync for Messenger<S> {}

type Buckets<S> = Vec<Vec<(Instant, Dispatch<S>)>>;

#[derive(Clone, Copy)]
pub struct Dispatch<S: SignalType> {
	pos: Option<(f32, f32)>,
	signal: S,
	delay: f32,
}

impl<S: SignalType> Messenger<S> {
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::channel();

		let mut global = Vec::new();
		global.resize_with(S::COUNT, Default::default);

		Self {
			now: Instant::now(),
			global,
			locals: Grid::new(256.),
			sender,
			receiver,
		}
	}

	pub fn sender(&self) -> mpsc::Sender<Dispatch<S>> {
		self.sender.clone()
	}

	pub fn update(&mut self, now: Instant) {
		self.now = now;

		let alive = |(time, dispatch): &(Instant, Dispatch<S>)| {
			now < (*time + Duration::from_secs_f32(dispatch.delay))
		};

		for bucket in &mut self.global {
			bucket.retain(alive)
		}

		self.locals.retain(alive);
		self.locals.maintain();

		while let Ok(dispatch) = self.receiver.try_recv() {
			if dispatch.pos.is_some() {
				self.locals.insert((self.now, dispatch));
			} else {
				let ty = dispatch.signal.kind();
				self.global[ty.into()].push((self.now, dispatch));
			}
		}
	}

	pub fn global_receive<'a>(
		&'a self,
		types: &'a [S::SignalKinds],
	) -> impl Iterator<Item = &'a S> + 'a {
		types
			.iter()
			.flat_map(|&ty| self.global[ty.into()].iter())
			.map(|(_, dispatch)| &dispatch.signal)
	}

	pub fn local_receive<'a>(
		&'a self,
		pos: Vector2<f32>,
		radius: f32,
		types: &'a [S::SignalKinds],
	) -> impl Iterator<Item = ((f32, f32), &'a S)> + 'a {
		self.locals
			.query_at(pos, radius)
			.map(|(_id, (_time, dispatch))| (dispatch.pos.unwrap(), &dispatch.signal))
			.filter(|(_, signal)| types.contains(&signal.kind()))
	}
}

impl<S: SignalType> Dispatch<S> {
	pub fn global(signal: S, delay: f32) -> Self {
		Self {
			pos: None,
			signal,
			delay,
		}
	}

	pub fn local(pos: (f32, f32), signal: S, delay: f32) -> Self {
		Self {
			pos: Some(pos),
			signal,
			delay,
		}
	}
}

impl<S: SignalType> Griddable for (Instant, Dispatch<S>) {
	fn pos(&self) -> (f32, f32) {
		self.1.pos.expect("Global dispatch.")
	}
}
