use once_cell::sync::Lazy;

use crate::{
	bindings::*, generate_bindings, include_doc
};
use std::os::raw::{c_int, c_void};
use crate::dword::AsDWORD;

static BASS_MIX_LIBRARY: Lazy<BASS_Mix> = Lazy::new(|| {
	#[cfg(target_os = "macos")]
		let library_name = "libbassmix.dylib";
	#[cfg(target_os = "windows")]
		let library_name = "bassmix.dll";
	#[cfg(target_os = "linux")]
		let library_name = "libbassmix.so";
	if let Ok(library) = unsafe { BASS_Mix::new(library_name) } {
		library
	} else {
		panic!("Failed to load the library.");
	}
});

// #[doc = include_str!("../doc/bassmix/BASS_Mixer_StreamCreate.html")]
// fn test() {
//
// }

generate_bindings! {
	BASS_MIX_LIBRARY;
	"/doc/bassmix/";
    // Info
    fn BASS_Mixer_GetVersion() -> DWORD;
    // Mixer Streams
	// #[doc = include_str!("../doc/bassmix/BASS_Mixer_StreamCreate.html")]
	// #[doc = include_str!(concat!(env!("OUT_DIR"), "/src/doc/bassmix/BASS_Mixer_StreamCreate.html"))]
    fn BASS_Mixer_StreamCreate(freq: DWORD, channels: DWORD, flags: DWORD) -> HSTREAM;
    fn BASS_Mixer_StreamAddChannel(handle: HSTREAM, channel: DWORD, flags: DWORD) -> BOOL;
    fn BASS_Mixer_StreamAddChannelEx(handle: HSTREAM, channel: DWORD, flags: DWORD, start: QWORD, length: QWORD) -> BOOL;
    fn BASS_Mixer_StreamGetChannels(handle: HSTREAM, channels: *mut DWORD, count: DWORD) -> DWORD;
    // Mixer Source Channels
    fn BASS_Mixer_ChannelFlags(handle.to(): impl AsDWORD, flags: DWORD, mask: DWORD) -> DWORD;
    fn BASS_Mixer_ChannelGetData(handle.to(): impl AsDWORD, buffer: *mut c_void, length: DWORD) -> DWORD;
    fn BASS_Mixer_ChannelGetEnvelopePos(handle.to(): impl AsDWORD, env_type: DWORD, value: *mut f32) -> QWORD;
    fn BASS_Mixer_ChannelGetLevel(handle.to(): impl AsDWORD) -> DWORD;
    fn BASS_Mixer_ChannelGetLevelEx(handle.to(): impl AsDWORD, levels: *mut f32, length: f32, flags: DWORD) -> BOOL;
    fn BASS_Mixer_ChannelGetMatrix(handle.to(): impl AsDWORD, matrix: *mut c_void) -> BOOL;
    fn BASS_Mixer_ChannelGetMixer(handle.to(): impl AsDWORD) -> HSTREAM;
    fn BASS_Mixer_ChannelGetPosition(handle.to(): impl AsDWORD, mode: DWORD) -> QWORD;
    fn BASS_Mixer_ChannelGetPositionEx(handle.to(): impl AsDWORD, mode: DWORD, delay: DWORD) -> QWORD;
    fn BASS_Mixer_ChannelIsActive(handle.to(): impl AsDWORD) -> DWORD;
    fn BASS_Mixer_ChannelRemove(handle.to(): impl AsDWORD) -> BOOL;
    fn BASS_Mixer_ChannelRemoveSync(handle.to(): impl AsDWORD, sync: HSYNC) -> BOOL;
    fn BASS_Mixer_ChannelSetEnvelope(handle.to(): impl AsDWORD, env_type: DWORD, nodes: *mut BASS_MIXER_NODE, count: DWORD) -> BOOL;
    fn BASS_Mixer_ChannelSetEnvelopePos(handle.to(): impl AsDWORD, env_type: DWORD, pos: QWORD) -> BOOL;
    fn BASS_Mixer_ChannelSetMatrix(handle.to(): impl AsDWORD, matrix: *mut c_void) -> BOOL;
    fn BASS_Mixer_ChannelSetMatrixEx(handle.to(): impl AsDWORD, matrix: *mut c_void, time: f32) -> BOOL;
    fn BASS_Mixer_ChannelSetPosition(handle.to(): impl AsDWORD, position: QWORD, mode: DWORD) -> BOOL;
    fn BASS_Mixer_ChannelSetSync(handle.to(): impl AsDWORD, sync_type: DWORD, param: QWORD, proc: SYNCPROC, user: *mut c_void) -> HSYNC;
    // Splitters
    fn BASS_Split_StreamCreate(channel: DWORD, flags: DWORD, chanmap: *const c_int) -> HSTREAM;
    fn BASS_Split_StreamGetAvailable(handle.to(): impl AsDWORD) -> DWORD;
    fn BASS_Split_StreamGetSource(handle: HSTREAM) -> DWORD;
    fn BASS_Split_StreamGetSplits(handle.to(): impl AsDWORD, splits: *mut HSTREAM, count: DWORD) -> DWORD;
    fn BASS_Split_StreamReset(handle.to(): impl AsDWORD) -> BOOL;
    fn BASS_Split_StreamResetEx(handle.to(): impl AsDWORD, offset: DWORD) -> BOOL;
}