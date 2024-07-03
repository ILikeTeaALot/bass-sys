mod bass;
#[cfg(all(any(target_os = "windows", target_os = "linux"), feature = "basscd"))]
mod bass_cd;
#[cfg(feature = "bassloud")]
mod bass_loud;
#[cfg(feature = "bassmix")]
mod bass_mix;
/// Config `#[cfg(all(feature = "basswasapi", target_os = "windows"))]`
#[cfg(all(feature = "basswasapi", target_os = "windows"))]
mod bass_wasapi;

pub use bass::*;
#[cfg(all(any(target_os = "windows", target_os = "linux"), feature = "basscd"))]
pub use bass_cd::*;
#[cfg(feature = "bassloud")]
pub use bass_loud::*;
#[cfg(feature = "bassmix")]
pub use bass_mix::*;
// pub use makelong::*;
/// Config: `#[cfg(all(feature = "basswasapi", target_os = "windows"))]`
#[cfg(all(feature = "basswasapi", target_os = "windows"))]
pub use bass_wasapi::*;