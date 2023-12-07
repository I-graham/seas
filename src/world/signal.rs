use strum::*;
use strum_macros::*;

#[derive(Clone, Copy, EnumDiscriminants, EnumCount)]
#[strum_discriminants(name(SignalTypes))]
pub enum Signal {
	Nil,
	BoatNearby,
}

impl super::messenger::SignalType for Signal {
	type SignalTypes = SignalTypes;
	const COUNT: usize = <Self as EnumCount>::COUNT;
}

impl From<SignalTypes> for usize {
	fn from(value: SignalTypes) -> Self {
		value as usize
	}
}
