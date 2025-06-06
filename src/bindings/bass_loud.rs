/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::bass::*;

pub const BASS_LOUDNESS_CURRENT: DWORD = DWORD(0);
pub const BASS_LOUDNESS_INTEGRATED: DWORD = DWORD(1);
pub const BASS_LOUDNESS_RANGE: DWORD = DWORD(2);
pub const BASS_LOUDNESS_PEAK: DWORD = DWORD(4);
pub const BASS_LOUDNESS_TRUEPEAK: DWORD = DWORD(8);
pub const BASS_LOUDNESS_AUTOFREE: DWORD = DWORD(32768);
#[repr(transparent)]
#[derive()]
pub struct HLOUDNESS(pub DWORD);
impl ::std::ops::Deref for HLOUDNESS {
	type Target = DWORD;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl ::std::ops::DerefMut for HLOUDNESS {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
pub struct BASS_Loud {
	__library: ::libloading::Library,
	pub BASS_Loudness_GetVersion: unsafe extern "C" fn() -> DWORD,
	pub BASS_Loudness_Start:
		unsafe extern "C" fn(handle: DWORD, flags: DWORD, priority: ::std::os::raw::c_int) -> HLOUDNESS,
	pub BASS_Loudness_Stop: unsafe extern "C" fn(handle: DWORD) -> BOOL,
	pub BASS_Loudness_SetChannel:
		unsafe extern "C" fn(handle: HLOUDNESS, channel: DWORD, priority: ::std::os::raw::c_int) -> BOOL,
	pub BASS_Loudness_GetChannel: unsafe extern "C" fn(handle: HLOUDNESS) -> DWORD,
	pub BASS_Loudness_GetLevel: unsafe extern "C" fn(handle: HLOUDNESS, mode: DWORD, level: *mut f32) -> BOOL,
	pub BASS_Loudness_GetLevelMulti:
		unsafe extern "C" fn(handles: *mut HLOUDNESS, count: DWORD, mode: DWORD, level: *mut f32) -> BOOL,
}
impl BASS_Loud {
	pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
	where
		P: AsRef<::std::ffi::OsStr>,
	{
		let library = ::libloading::Library::new(path)?;
		Self::from_library(library)
	}
	pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
	where
		L: Into<::libloading::Library>,
	{
		let __library = library.into();
		let BASS_Loudness_GetVersion = __library.get(b"BASS_Loudness_GetVersion\0").map(|sym| *sym)?;
		let BASS_Loudness_Start = __library.get(b"BASS_Loudness_Start\0").map(|sym| *sym)?;
		let BASS_Loudness_Stop = __library.get(b"BASS_Loudness_Stop\0").map(|sym| *sym)?;
		let BASS_Loudness_SetChannel = __library.get(b"BASS_Loudness_SetChannel\0").map(|sym| *sym)?;
		let BASS_Loudness_GetChannel = __library.get(b"BASS_Loudness_GetChannel\0").map(|sym| *sym)?;
		let BASS_Loudness_GetLevel = __library.get(b"BASS_Loudness_GetLevel\0").map(|sym| *sym)?;
		let BASS_Loudness_GetLevelMulti = __library.get(b"BASS_Loudness_GetLevelMulti\0").map(|sym| *sym)?;
		Ok(BASS_Loud {
			__library,
			BASS_Loudness_GetVersion,
			BASS_Loudness_Start,
			BASS_Loudness_Stop,
			BASS_Loudness_SetChannel,
			BASS_Loudness_GetChannel,
			BASS_Loudness_GetLevel,
			BASS_Loudness_GetLevelMulti,
		})
	}
	pub unsafe fn BASS_Loudness_GetVersion(&self) -> DWORD {
		(self.BASS_Loudness_GetVersion)()
	}
	pub unsafe fn BASS_Loudness_Start(
		&self,
		handle: DWORD,
		flags: DWORD,
		priority: ::std::os::raw::c_int,
	) -> HLOUDNESS {
		(self.BASS_Loudness_Start)(handle, flags, priority)
	}
	pub unsafe fn BASS_Loudness_Stop(&self, handle: DWORD) -> BOOL {
		(self.BASS_Loudness_Stop)(handle)
	}
	pub unsafe fn BASS_Loudness_SetChannel(
		&self,
		handle: HLOUDNESS,
		channel: DWORD,
		priority: ::std::os::raw::c_int,
	) -> BOOL {
		(self.BASS_Loudness_SetChannel)(handle, channel, priority)
	}
	pub unsafe fn BASS_Loudness_GetChannel(&self, handle: HLOUDNESS) -> DWORD {
		(self.BASS_Loudness_GetChannel)(handle)
	}
	pub unsafe fn BASS_Loudness_GetLevel(&self, handle: HLOUDNESS, mode: DWORD, level: *mut f32) -> BOOL {
		(self.BASS_Loudness_GetLevel)(handle, mode, level)
	}
	pub unsafe fn BASS_Loudness_GetLevelMulti(
		&self,
		handles: *mut HLOUDNESS,
		count: DWORD,
		mode: DWORD,
		level: *mut f32,
	) -> BOOL {
		(self.BASS_Loudness_GetLevelMulti)(handles, count, mode, level)
	}
}
