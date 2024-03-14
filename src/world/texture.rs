use super::*;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Debug, Eq, Clone, Copy)]
pub enum Texture {
	Flat,
	Puffin,
	PuffinFlap,
	PuffinFly,
	PuffinFlip,
	PuffinPeck,
	Raft,
	Wave,
	Node,
}

impl TextureType for Texture {
	fn list() -> Vec<Self> {
		use strum::IntoEnumIterator;
		Self::iter().collect()
	}

	fn flat() -> Self {
		Self::Flat
	}

	fn name(&self) -> &'static str {
		self.into()
	}

	fn frame_count(&self) -> u32 {
		use Texture::*;
		match self {
			PuffinFlap => 5,
			PuffinFly => 8,
			PuffinPeck => 4,
			Wave => 27,
			_ => 1,
		}
	}
}
