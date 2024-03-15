//A simple struct to offload a computation to another thread
//and block until it is complete

use std::cell::OnceCell;
use std::sync::mpsc::*;

pub struct Task<T: Send + 'static> {
	result: OnceCell<T>,
	recv: Option<Receiver<T>>,
}

impl<T: Send + 'static> Task<T> {
	//Construct without launching thread
	pub fn from_val(val: T) -> Self {
		let cell = OnceCell::new();

		let _ = cell.set(val);

		Self {
			result: cell,
			recv: None,
		}
	}

	pub fn launch(f: impl FnOnce() -> T + Send + 'static) -> Self {
		let (sender, receiver) = channel();

		rayon::spawn(move || {
			let _ = sender.send(f());
		});

		Self {
			result: OnceCell::new(),
			recv: Some(receiver),
		}
	}

	pub fn if_done(&self) -> Option<&T> {
		if let Some(recv) = &self.recv {
			if let Ok(result) = recv.try_recv() {
				let _ = self.result.set(result);
			}
		}

		self.result.get()
	}

	pub fn if_done_mut(&mut self) -> Option<&mut T> {
		if let Some(recv) = &self.recv {
			if let Ok(result) = recv.try_recv() {
				let _ = self.result.set(result);
			}
		}

		self.result.get_mut()
	}

	pub fn get(&self) -> &T {
		if self.result.get().is_none() {
			let result = self.recv.as_ref().unwrap().recv().unwrap();
			let _ = self.result.set(result);
		}

		self.result.get().unwrap()
	}

	pub fn get_mut(&mut self) -> &mut T {
		if self.result.get().is_none() {
			let result = self.recv.as_ref().unwrap().recv().unwrap();
			let _ = self.result.set(result);
		}
		self.result.get_mut().unwrap()
	}
}
