pub use bindings::*;
#[allow(unused)]
pub use consts::*;
pub use functions::*;
pub use types::*;

mod bindings;
pub mod functions;
mod impls;
mod macros;

pub use impls::*;

pub mod consts {
	pub use crate::bindings::*;
}

pub mod types {
	use std::ffi::c_char;

	pub use crate::bindings::*;

	// pub type BassDeviceInfo = BASS_DEVICEINFO;

	impl BASS_DEVICEINFO {
		pub fn new(name: *const c_char, driver: *const c_char, flags: impl Into<DWORD>) -> Self {
			Self { name, driver, flags: flags.into() }
		}
	}

	// pub type BassInfo = BASS_INFO;

	// pub type BassRecordInfo = BASS_RECORDINFO;

	// pub type BassSample = BASS_SAMPLE;

	// // pub type BassChannelInfo = BASS_CHANNELINFO;

	// pub type BassPluginForm = BASS_PLUGINFORM;

	// pub type BassPluginInfo = BASS_PLUGININFO;

	// pub type Bass3DVector = BASS_3DVECTOR;

	// pub type BassFileProcs = BASS_FILEPROCS;
}

#[cfg(test)]
mod tests {
	#[cfg(target_os = "macos")]
	#[test]
	fn test_lib_path() {
		use crate::bass_path_prefix;

		#[cfg(not(any(feature = "rpath", feature = "executable_path", feature = "loader_path")))]
		assert_eq!(bass_path_prefix(), "");
		#[cfg(feature = "rpath")]
		assert_eq!(bass_path_prefix(), "@rpath/");
		#[cfg(feature = "executable_path")]
		assert_eq!(bass_path_prefix(), "@executable_path/");
		#[cfg(feature = "loader_path")]
		assert_eq!(bass_path_prefix(), "@loader_path/");
	}
}
