use strum::*;
use strum_macros::*;

#[derive(Clone, Copy, EnumDiscriminants, EnumCount)]
#[strum_discriminants(name(SignalKinds))]
pub enum Signal {
	Nil,
	BoatNearby,
}

impl super::messenger::SignalType for Signal {
	type SignalKinds = SignalKinds;
	const COUNT: usize = <Self as EnumCount>::COUNT;
}

impl From<SignalKinds> for usize {
	fn from(value: SignalKinds) -> Self {
		value as usize
	}
}
