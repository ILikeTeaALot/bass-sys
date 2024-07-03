use std::cmp::Ordering;
use std::{
	fmt::{Binary, Display},
	ops::{BitAnd, BitOr, BitXor, Bound, Deref, Not, RangeBounds},
};

use crate::{DWORD, HCHANNEL, HDSP, HFX, HLOUDNESS, HMUSIC, HPLUGIN, HRECORD, HSAMPLE, HSTREAM, HSYNC};

macro_rules! generate_impls {
	($item: ident) => {
		// impl IntoDWORD for $item {
		// 	fn to(self) -> DWORD {
		// 		self.0
		// 	}
		// }

		impl From<$item> for DWORD {
			fn from(value: $item) -> Self {
				value.0
			}
		}

		impl PartialEq<u32> for $item {
			fn eq(&self, other: &u32) -> bool {
				self.0 == *other
			}

			fn ne(&self, other: &u32) -> bool {
				self.0 != *other
			}
		}
	};
}

// BASS Plugin handle
generate_impls!(HPLUGIN);
// MOD/MO3 handles
generate_impls!(HMUSIC);
generate_impls!(HSAMPLE);
generate_impls!(HCHANNEL);
// Recording "Channel" handle
generate_impls!(HRECORD);
// File/Other stream handle
generate_impls!(HSTREAM);
// SyncProc Sync handle
generate_impls!(HSYNC);
// BASS_Fx_* Effects
generate_impls!(HFX);
// DSP effects
generate_impls!(HDSP);
#[cfg(feature = "bassloud")]
generate_impls!(HLOUDNESS);

impl Display for DWORD {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("DWORD({})", self.0))
	}
}

impl From<u32> for DWORD {
	fn from(value: u32) -> Self {
		DWORD(value)
	}
}

impl From<i32> for DWORD {
	fn from(value: i32) -> Self {
		DWORD(value as u32)
	}
}

impl From<usize> for DWORD {
	fn from(value: usize) -> Self {
		DWORD(value as u32)
	}
}

impl PartialOrd<u32> for DWORD {
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		u32::partial_cmp(&self.0, other)
	}
}

impl PartialOrd<DWORD> for DWORD {
	fn partial_cmp(&self, other: &DWORD) -> Option<std::cmp::Ordering> {
		u32::partial_cmp(&self.0, &other.0)
	}
}

impl Ord for DWORD {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(other)
	}
}

impl Not for DWORD {
	type Output = DWORD;

	fn not(self) -> Self::Output {
		DWORD(!self.0)
	}
}

impl BitXor for DWORD {
	type Output = DWORD;

	fn bitxor(self, rhs: Self) -> Self::Output {
		DWORD(self.0 ^ rhs.0)
	}
}

impl PartialEq<u32> for DWORD {
	fn eq(&self, other: &u32) -> bool {
		self.0 == *other
	}

	fn ne(&self, other: &u32) -> bool {
		self.0 != *other
	}
}

impl PartialEq<i32> for DWORD {
	fn eq(&self, other: &i32) -> bool {
		self.0 == *other as u32
	}

	fn ne(&self, other: &i32) -> bool {
		self.0 != *other as u32
	}
}

impl BitOr<u32> for DWORD {
	type Output = u32;

	fn bitor(self, rhs: u32) -> Self::Output {
		self.0 | rhs
	}
}

impl BitOr for DWORD {
	type Output = DWORD;

	fn bitor(self, rhs: DWORD) -> Self::Output {
		bitor(self, rhs)
	}
}

/// Utility const fn for BitOR-ing DWORD for flags.
///
/// Hopefully it won't be too long before `const impl`
/// is stabilised...
pub const fn bitor(lhs: DWORD, rhs: DWORD) -> DWORD {
	DWORD(lhs.0 | rhs.0)
}

impl BitAnd<u32> for DWORD {
	type Output = u32;

	fn bitand(self, rhs: u32) -> Self::Output {
		self.0 & rhs
	}
}

impl BitAnd<DWORD> for DWORD {
	type Output = DWORD;

	fn bitand(self, rhs: DWORD) -> Self::Output {
		DWORD(self.0 & rhs.0)
	}
}
