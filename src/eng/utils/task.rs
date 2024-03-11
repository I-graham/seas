//A simple struct to offload a computation to another thread
//and block until it is complete

use std::cell::{Cell, OnceCell};
use std::thread;

pub struct Task<T: Send + 'static> {
	result: OnceCell<T>,
	status: Cell<Option<thread::JoinHandle<T>>>,
}

impl<T: Send + 'static> Task<T> {
	//Construct without launching thread
	pub fn from_val(val: T) -> Self {
		let cell = OnceCell::new();

		let _ = cell.set(val);

		Self {
			result: cell,
			status: Cell::new(None),
		}
	}

	pub fn launch(f: impl FnOnce() -> T + Send + 'static) -> Self {
		let task = thread::spawn(f);
		Self {
			result: OnceCell::new(),
			status: Cell::new(Some(task)),
		}
	}

	pub fn if_done(&self) -> Option<&T> {
		if let Some(handle) = self.status.take() {
			if handle.is_finished() {
				let result = handle.join().unwrap();
				let _ = self.result.set(result);
			} else {
				self.status.set(Some(handle));
			}
		};

		self.result.get()
	}

	pub fn if_done_mut(&mut self) -> Option<&mut T> {
		if let Some(handle) = self.status.take() {
			if handle.is_finished() {
				let result = handle.join().unwrap();
				let _ = self.result.set(result);
			} else {
				self.status.set(Some(handle));
			}
		};

		self.result.get_mut()
	}

	pub fn get(&self) -> &T {
		if let Some(handle) = self.status.take() {
			let result = handle.join().unwrap();
			let _ = self.result.set(result);
		}

		self.result.get().unwrap()
	}

	pub fn get_mut(&mut self) -> &mut T {
		if let Some(handle) = self.status.take() {
			let result = handle.join().unwrap();
			let _ = self.result.set(result);
		}

		self.result.get_mut().unwrap()
	}
}
