use strum::*;
use strum_macros::*;

#[derive(EnumDiscriminants, EnumCount)]
#[strum_discriminants(name(SignalKinds))]
pub enum Signal {
	BoatNearby,
	Nil,
}

impl super::messenger::SignalType for Signal {
	type SignalKinds = SignalKinds;
	const COUNT: usize = <Self as EnumCount>::COUNT;

	fn kind(&self) -> Self::SignalKinds {
		SignalKinds::from(self)
	}
}

impl From<SignalKinds> for usize {
	fn from(value: SignalKinds) -> Self {
		value as usize
	}
}
