mod bass;
#[cfg(all(any(target_os = "windows", target_os = "linux"), feature = "basscd"))]
mod bass_cd;
#[cfg(feature = "bassloud")]
mod bass_loud;
#[cfg(feature = "bassmix")]
mod bass_mix;
pub mod makelong;

pub use bass::*;
#[cfg(all(any(target_os = "windows", target_os = "linux"), feature = "basscd"))]
pub use bass_cd::*;
#[cfg(feature = "bassloud")]
pub use bass_loud::*;
#[cfg(feature = "bassmix")]
pub use bass_mix::*;
// pub use makelong::*;