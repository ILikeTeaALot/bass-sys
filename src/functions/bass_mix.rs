use once_cell::sync::Lazy;

use crate::{
	bindings::*, generate_bindings
};
use std::os::raw::{c_int, c_void};

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

generate_bindings! {
	BASS_MIX_LIBRARY;
	"/doc/bassmix/";
    // Info
    pub fn BASS_Mixer_GetVersion() -> DWORD;
    // Mixer Streams
	// #[doc = include_str!("../doc/bassmix/BASS_Mixer_StreamCreate.html")]
	// #[doc = include_str!(concat!(env!("OUT_DIR"), "/src/doc/bassmix/BASS_Mixer_StreamCreate.html"))]
    pub fn BASS_Mixer_StreamCreate(freq.into(): impl Into<DWORD>, channels.into(): impl Into<DWORD>, flags.into(): impl Into<DWORD>) -> HSTREAM;
    pub fn BASS_Mixer_StreamAddChannel(handle: HSTREAM, channel.into(): impl Into<DWORD>, flags.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_Mixer_StreamAddChannelEx(handle: HSTREAM, channel.into(): impl Into<DWORD>, flags.into(): impl Into<DWORD>, start.into(): impl Into<QWORD>, length.into(): impl Into<QWORD>) -> bool;
    /// # Safety
    ///
    /// Be **VERY** careful with `channels`, as it requires manually handling the sizing, allocation, and resizing of a Vec or similar.
    ///
    /// ## Example of Correct Usage
    ///
    /// ```
	/// use std::ptr::null_mut;
	/// use bass_sys::{DWORD, HSTREAM};
	/// use bass_sys::BASS_Mixer_StreamGetChannels;
	///
	/// /// [Example] Get the channels in a Mixer channel.
	/// pub fn get_channels(handle: HSTREAM) -> Vec<DWORD> {
	/// 	// Get the number of handles currently "in" the mixer. As per BASS docs, it is safe to pass NULL when not requesting any channels be inserted.
	/// 	let count = unsafe { BASS_Mixer_StreamGetChannels(handle, null_mut::<DWORD>(), 0) };
	/// 	// Initialise a Vec<DWORD> with the appropriate pre-allocated space.
	/// 	let mut channels = Vec::with_capacity(*count as usize);
	/// 	// Give BASS a pointer to our `channels` vector, and get the number of handles inserted.
	/// 	let inserted = unsafe { BASS_Mixer_StreamGetChannels(handle, channels.as_mut_ptr(), count) };
	/// 	// Vec length can only be set in unsafe code.
	/// 	unsafe {
	/// 		channels.set_len(*inserted as usize) // Use the returned "inserted" count in case less handles than expected were inserted.
	/// 	}
	/// 	return channels;
	/// }
	/// ```
    pub most unsafe fn BASS_Mixer_StreamGetChannels(handle: HSTREAM, channels: *mut DWORD, count.into(): impl Into<DWORD>) -> DWORD;
    // Mixer Source Channels
    pub fn BASS_Mixer_ChannelFlags(handle.into(): impl Into<DWORD>, flags.into(): impl Into<DWORD>, mask.into(): impl Into<DWORD>) -> DWORD;
    pub most unsafe fn BASS_Mixer_ChannelGetData(handle.into(): impl Into<DWORD>, buffer: *mut c_void, length.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_Mixer_ChannelGetEnvelopePos(handle.into(): impl Into<DWORD>, env_type.into(): impl Into<DWORD>, value: &mut f32 as *mut f32) -> QWORD;
    pub fn BASS_Mixer_ChannelGetLevel(handle.into(): impl Into<DWORD>) -> DWORD;
    pub most unsafe fn BASS_Mixer_ChannelGetLevelEx(handle.into(): impl Into<DWORD>, levels: *mut f32, length: f32, flags.into(): impl Into<DWORD>) -> bool;
    pub most unsafe fn BASS_Mixer_ChannelGetMatrix(handle.into(): impl Into<DWORD>, matrix: *mut c_void) -> bool;
    pub fn BASS_Mixer_ChannelGetMixer(handle.into(): impl Into<DWORD>) -> HSTREAM;
    pub fn BASS_Mixer_ChannelGetPosition(handle.into(): impl Into<DWORD>, mode.into(): impl Into<DWORD>) -> QWORD;
    pub fn BASS_Mixer_ChannelGetPositionEx(handle.into(): impl Into<DWORD>, mode.into(): impl Into<DWORD>, delay.into(): impl Into<DWORD>) -> QWORD;
    pub fn BASS_Mixer_ChannelIsActive(handle.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_Mixer_ChannelRemove(handle.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_Mixer_ChannelRemoveSync(handle.into(): impl Into<DWORD>, sync: HSYNC) -> bool;
    pub fn BASS_Mixer_ChannelSetEnvelope(handle.into(): impl Into<DWORD>, env_type.into(): impl Into<DWORD>, nodes: *mut BASS_MIXER_NODE, count.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_Mixer_ChannelSetEnvelopePos(handle.into(): impl Into<DWORD>, env_type.into(): impl Into<DWORD>, pos.into(): impl Into<QWORD>) -> bool;
    pub most unsafe fn BASS_Mixer_ChannelSetMatrix(handle.into(): impl Into<DWORD>, matrix: *mut c_void) -> bool;
    pub most unsafe fn BASS_Mixer_ChannelSetMatrixEx(handle.into(): impl Into<DWORD>, matrix: *mut c_void, time: f32) -> bool;
    pub fn BASS_Mixer_ChannelSetPosition(handle.into(): impl Into<DWORD>, position.into(): impl Into<QWORD>, mode.into(): impl Into<DWORD>) -> bool;
    pub most unsafe fn BASS_Mixer_ChannelSetSync(handle.into(): impl Into<DWORD>, sync_type.into(): impl Into<DWORD>, param.into(): impl Into<QWORD>, proc: SYNCPROC, user: *mut c_void) -> HSYNC;
    // Splitters
    pub most unsafe fn BASS_Split_StreamCreate(channel.into(): impl Into<DWORD>, flags.into(): impl Into<DWORD>, chanmap: *const c_int) -> HSTREAM;
    pub fn BASS_Split_StreamGetAvailable(handle.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_Split_StreamGetSource(handle: HSTREAM) -> DWORD;
    pub fn BASS_Split_StreamGetSplits(handle.into(): impl Into<DWORD>, splits: &mut HSTREAM as *mut HSTREAM, count.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_Split_StreamReset(handle.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_Split_StreamResetEx(handle.into(): impl Into<DWORD>, offset.into(): impl Into<DWORD>) -> bool;
}