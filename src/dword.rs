use crate::{HLOUDNESS, DWORD, HCHANNEL, HDSP, HFX, HMUSIC, HPLUGIN, HRECORD, HSAMPLE, HSTREAM, HSYNC};

macro_rules! generate_impls {
	($item: ident) => {
		impl AsDWORD for $item {
			fn to(self) -> DWORD {
				self.0
			}
		}

		impl Into<DWORD> for $item {
			fn into(self) -> DWORD {
				self.0
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

/// To make type-checking easier, the different handles in BASS are treated as transparent
/// NewTypes in the Rust-world.
///
/// This trait exists to allow functions that accept many different types of handle
/// (i.e. [`BASS_ChannelPause`](crate::BASS_ChannelPause)) to seamlessly accept any
/// compatible `H*` type.
///
/// You're encouraged to use this type in your own code (as `param: impl AsDWORD`) in-place of `channel: DWORD`
///
/// ```
/// use bass_sys::*;
/// use std::ptr::null_mut;
///
/// fn main() {
///     let stream: HSTREAM = BASS_StreamCreate(44100, 2, BASS_SAMPLE_FLOAT, None, null_mut()); // -> struct HSTREAM(pub DWORD);
///     let ok = BASS_ChannelPlay(stream, FALSE); // Automatically "unwrapped" to the underlying DWORD
///     if !ok {
///         // This will happen in the example because the Stream has no content.
///         eprintln!("Failed to play channel!");
/// 	}
/// }
/// ```
///
/// ```
/// use bass_sys::*;
///
/// fn main() -> Result<(), DWORD> {
///     let sample = BASS_SampleCreate(10, 44100, 2, 10, 0);
///     do_a_thing(sample)?;
///     Ok(())
/// }
///
/// fn do_a_thing(channel: impl AsDWORD) -> Result<(), DWORD> {
///     match BASS_ChannelPause(channel.to()) {
///         BOOL::FALSE => Err(BASS_ErrorGetCode() as DWORD),
/// 		BOOL::TRUE => Ok(())
/// 	}
/// }
/// ```
pub trait AsDWORD {
	fn to(self) -> DWORD;
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
// BASS_Fx Effects
generate_impls!(HFX);
// DSP effects
generate_impls!(HDSP);
#[cfg(feature = "bassloud")]
generate_impls!(HLOUDNESS);

// impl AsDWORD for DWORD {
// 	fn to(self) -> DWORD {
// 		self
// 	}
// }

impl AsDWORD for u32 {
	fn to(self) -> DWORD {
		self
	}
}

impl AsDWORD for i32 {
	fn to(self) -> DWORD {
		self as DWORD
	}
}

// impl<T> AsDWORD for T where T: Deref<Target = DWORD> {
// 	fn to(self) -> DWORD {
// 		self.clone() as u32 as DWORD
// 	}
// }