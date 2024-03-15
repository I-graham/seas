use std::sync::mpsc;
use std::time::*;

use super::{Grid, Griddable};

pub trait SignalType: Copy {
	type SignalKinds: Copy + From<Self> + Into<usize> + PartialEq;
	const COUNT: usize;
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
		Self {
			now: Instant::now(),
			global: vec![Default::default(); S::COUNT],
			locals: Grid::new(128.),
			sender,
			receiver,
		}
	}

	pub fn sender(&self) -> mpsc::Sender<Dispatch<S>> {
		self.sender.clone()
	}

	pub fn update(&mut self, now: Instant) {
		self.now = now;

		let alive = |&(time, dispatch): &(Instant, Dispatch<S>)| {
			now < (time + Duration::from_secs_f32(dispatch.delay))
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
				let ty = S::SignalKinds::from(dispatch.signal);
				self.global[ty.into()].push((self.now, dispatch));
			}
		}
	}

	pub fn global_receive<'a>(
		&'a self,
		types: &'a [S::SignalKinds],
	) -> impl Iterator<Item = S> + 'a {
		types
			.iter()
			.flat_map(|&ty| self.global[ty.into()].iter())
			.map(|(_, dispatch)| dispatch.signal)
	}

	pub fn local_receive<'a>(
		&'a self,
		pos: (f32, f32),
		radius: f32,
		types: &'a [S::SignalKinds],
	) -> impl Iterator<Item = ((f32, f32), S)> + 'a {
		self.locals
			.query_at(pos, radius)
			.map(|(_id, &dispatch)| (dispatch.pos(), dispatch.1.signal))
			.filter(|(_, signal)| types.contains(&S::SignalKinds::from(*signal)))
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
