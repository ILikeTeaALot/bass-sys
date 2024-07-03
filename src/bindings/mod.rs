mod bass;
/// Config `#[cfg(all(feature = "basscd", any(target_os = "windows", target_os = "linux")))]`
#[cfg(all(feature = "basscd", any(target_os = "windows", target_os = "linux")))]
mod bass_cd;
#[cfg(feature = "bassloud")]
mod bass_loud;
#[cfg(feature = "bassmix")]
mod bass_mix;
/// Config `#[cfg(all(feature = "basswasapi", target_os = "windows"))]`
#[cfg(all(feature = "basswasapi", target_os = "windows"))]
mod bass_wasapi;
pub (crate) mod makelong;

pub use bass::*;
/// Config: `#[cfg(all(feature = "basscd", any(target_os = "windows", target_os = "linux")))]`
#[cfg(all(feature = "basscd", any(target_os = "windows", target_os = "linux")))]
pub use bass_cd::*;
#[cfg(feature = "bassloud")]
pub use bass_loud::*;
#[cfg(feature = "bassmix")]
pub use bass_mix::*;
/// Config: `#[cfg(all(feature = "basswasapi", target_os = "windows"))]`
#[cfg(all(feature = "basswasapi", target_os = "windows"))]
pub use bass_wasapi::*;