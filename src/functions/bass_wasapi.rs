use once_cell::sync::Lazy;

use crate::{
	bindings::*, generate_bindings
};
use std::os::raw::{c_float, c_int, c_void};

static BASS_WASAPI_LIBRARY: Lazy<BASS_WASAPI> = Lazy::new(|| {
	#[cfg(target_os = "windows")]
		let library_name = "basswasapi.dll";
	if let Ok(library) = unsafe { BASS_WASAPI::new(library_name) } {
		library
	} else {
		panic!("Failed to load the library.");
	}
});

generate_bindings! {
	BASS_WASAPI_LIBRARY;
	"/doc/basswasapi/";
    // Info
    pub fn BASS_WASAPI_GetVersion() -> DWORD;
    pub fn BASS_WASAPI_GetCPU() -> c_float;
	// Init
	pub fn BASS_WASAPI_Free() -> bool;
	pub fn BASS_WASAPI_GetInfo(info: &mut BASS_WASAPI_INFO as *mut BASS_WASAPI_INFO) -> bool;
	pub most unsafe fn BASS_WASAPI_Init(
		device: c_int,
		freq.into(): impl Into<DWORD>,
		chans.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>,
		buffer: c_float,
		period: c_float,
		proc: WASAPIPROC,
		user: *mut c_void) -> bool;
	pub fn BASS_WASAPI_IsStarted() -> bool;
	pub fn BASS_WASAPI_Lock(lock.into(): bool) -> bool;
	pub fn BASS_WASAPI_Start() -> bool;
	pub fn BASS_WASAPI_Stop(reset.into(): bool) -> bool;
	// Device/Init/Other
	pub most unsafe fn BASS_WASAPI_SetNotify(proc: WASAPINOTIFYPROC, user: *mut c_void) -> bool;
	pub fn BASS_WASAPI_CheckFormat(
		device.into(): i32,
		freq.into(): impl Into<DWORD>,
		chans.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>) -> DWORD;
	// Device
	pub fn BASS_WASAPI_GetDevice() -> DWORD;
	pub fn BASS_WASAPI_GetDeviceInfo(device.into(): impl Into<DWORD>, info: &mut BASS_WASAPI_DEVICEINFO as *mut BASS_WASAPI_DEVICEINFO) -> DWORD;
	pub fn BASS_WASAPI_GetDeviceLevel(device.into(): impl Into<DWORD>, chan: c_int) -> c_float;
	pub fn BASS_WASAPI_GetMute(mode.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_WASAPI_GetVolume(device.into(): impl Into<DWORD>) -> c_float;
	pub fn BASS_WASAPI_SetDevice(device.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_WASAPI_SetMute(device.into(): impl Into<DWORD>, mute.into(): bool) -> bool;
	pub fn BASS_WASAPI_SetVolume(device.into(): impl Into<DWORD>, volume: c_float) -> bool;
	// Level
	pub fn BASS_WASAPI_GetLevel() -> DWORD;
	pub most unsafe fn BASS_WASAPI_GetLevelEx(levels: *mut c_float, length: c_float, flags.into(): impl Into<DWORD>) -> bool;
	// Data
	pub most unsafe fn BASS_WASAPI_GetData(buffer: *mut c_void, length.into(): impl Into<DWORD>) -> DWORD;
	pub most unsafe fn BASS_WASAPI_PutData(buffer: *mut c_void, length.into(): impl Into<DWORD>) -> DWORD;
}