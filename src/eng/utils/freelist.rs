use rayon::iter::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreeListEntryId(usize, usize);

pub struct FreeList<T> {
	inner: Vec<Elem<T>>,
	free: Option<usize>,
	id_counter: usize,
}

impl<T> FreeList<T> {
	pub fn new() -> Self {
		Self {
			inner: vec![],
			free: None,
			id_counter: 0,
		}
	}

	pub fn get(&self, index: FreeListEntryId) -> Option<&T> {
		let FreeListEntryId(id, index) = index;

		match &self.inner[index] {
			Elem::Entry(eid, item) if id == *eid => Some(item),
			Elem::Entry(_, _) => None,
			_ => panic!("Attempted to access empty freelist slot."),
		}
	}

	pub fn get_mut(&mut self, index: FreeListEntryId) -> Option<&mut T> {
		let FreeListEntryId(id, index) = index;

		match &mut self.inner[index] {
			Elem::Entry(eid, item) if id == *eid => Some(item),
			Elem::Entry(_, _) => None,
			_ => panic!("Attempted to access empty freelist slot."),
		}
	}

	pub fn slot_count(&self) -> usize {
		self.inner.capacity()
	}

	pub fn count(&self) -> usize {
		self.inner
			.iter()
			.filter(|e| matches!(e, Elem::Entry(_, _)))
			.count()
	}

	pub fn insert(&mut self, item: T) -> FreeListEntryId {
		let id = self.id_counter;
		self.id_counter += 1;

		if let Some(first_free) = self.free {
			self.free = match self.inner[first_free] {
				Elem::NextFree(next) => Some(next),
				Elem::LastFree => None,
				Elem::Entry(_, _) => unreachable!(),
			};
			self.inner[first_free] = Elem::Entry(id, item);
			FreeListEntryId(id, first_free)
		} else {
			self.inner.push(Elem::Entry(id, item));
			let slot = self.inner.len() - 1;
			FreeListEntryId(id, slot)
		}
	}

	pub fn remove(&mut self, index: FreeListEntryId) -> Option<T> {
		let FreeListEntryId(id, index) = index;

		match &mut self.inner[index] {
			Elem::Entry(eid, _) if id == *eid => {
				let obj = std::mem::replace(&mut self.inner[index], Elem::LastFree);

				if let Some(free) = self.free {
					self.inner[index] = Elem::NextFree(free);
				};

				self.free = Some(index);

				let Elem::Entry(_, obj) = obj else {
					unreachable!()
				};

				Some(obj)
			}
			_ => None,
		}
	}

	pub fn sort_frees(&mut self) {
		use Elem::*;
		if self.free.is_some() {
			let mut i = 0;
			while let Entry(_, _) = self.inner[i] {
				i += 1;
			}
			self.free = Some(i);

			let mut last_free = i;
			while i < self.inner.len() {
				match self.inner[i] {
					NextFree(_) | LastFree => {
						self.inner[last_free] = NextFree(i);
						last_free = i
					}
					_ => {}
				}
				i += 1;
			}

			self.inner[last_free] = LastFree;
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.inner.iter().filter_map(|elem| match elem {
			Elem::Entry(_, item) => Some(item),
			_ => None,
		})
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.inner.iter_mut().filter_map(|elem| match elem {
			Elem::Entry(_, item) => Some(item),
			_ => None,
		})
	}
}

impl<T: Send + Sync> FreeList<T> {
	pub fn par_iter(&self) -> impl ParallelIterator<Item = &T> {
		self.inner.par_iter().filter_map(|elem| match elem {
			Elem::Entry(_, item) => Some(item),
			_ => None,
		})
	}

	pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T> {
		self.inner.par_iter_mut().filter_map(|elem| match elem {
			Elem::Entry(_, item) => Some(item),
			_ => None,
		})
	}
}

impl<T> Index<FreeListEntryId> for FreeList<T> {
	type Output = T;
	fn index(&self, index: FreeListEntryId) -> &Self::Output {
		let FreeListEntryId(id, index) = index;

		match &self.inner[index] {
			Elem::Entry(eid, item) if id == *eid => item,
			Elem::Entry(_, _) => panic!("Item no longer exists"),
			_ => panic!("Attempted to access empty freelist slot."),
		}
	}
}

impl<T> IndexMut<FreeListEntryId> for FreeList<T> {
	fn index_mut(&mut self, index: FreeListEntryId) -> &mut Self::Output {
		let FreeListEntryId(id, index) = index;

		match &mut self.inner[index] {
			Elem::Entry(eid, item) if id == *eid => item,
			Elem::Entry(_, _) => panic!("Item no longer exists"),
			_ => panic!("Attempted to access empty freelist slot."),
		}
	}
}

enum Elem<T> {
	Entry(usize, T),
	NextFree(usize),
	LastFree,
}
