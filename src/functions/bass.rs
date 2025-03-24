use once_cell::sync::Lazy;

use crate::{
	// types::{
	//     Bass3DVector, BassChannelInfo, BassDeviceInfo, BassFileProcs, BassInfo, BassPluginInfo,
	//     BassRecordInfo, BassSample, BOOL, DOWNLOADPROC, DSPPROC, DWORD, HCHANNEL, HDSP, HFX,
	//     HMUSIC, HPLUGIN, HRECORD, HSAMPLE, HSTREAM, HSYNC, QWORD, RECORDPROC, STREAMPROC, SYNCPROC,
	// },
	bindings::*,
	generate_bindings,
};
use std::os::raw::{c_char, c_int, c_void};

static BASS_LIBRARY: Lazy<BASS> = Lazy::new(|| {
	#[cfg(target_os = "macos")]
	let library_name = "@rpath/libbass.dylib";
	#[cfg(target_os = "windows")]
	let library_name = "bass.dll";
	#[cfg(target_os = "linux")]
	let library_name = "libbass.so";
	// if let Ok(library) = unsafe { Library::new(library_name) } {
	//     return library;
	// } else {
	//     panic!("Failed to load the library.");
	// }
	if let Ok(library) = unsafe { BASS::new(library_name) } {
		library
	} else {
		panic!("Failed to load the library.");
	}
});

// #[repr(transparent)]
//pub union AsDWORD {
//     u32: u32,
//     i32: i32,
//     DWORD.into(): impl Into<DWORD>,
//     MUSIC: HMUSIC,
//     SAMPLE: HSAMPLE,
//     CHANNEL: HCHANNEL,
//     STREAM: HSTREAM,
//     RECORD: HRECORD,
//     SYNC: HSYNC,
//     DSP: HDSP,
//     FX: HFX,
//     PLUGIN: HPLUGIN,
// }

generate_bindings! {
	BASS_LIBRARY;
	"/doc/bass/";
	pub fn BASS_SetConfig(option.into(): impl Into<DWORD>, value.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_GetConfig(option.into(): impl Into<DWORD>) -> DWORD;
	/// # Safety
	///
	/// Due to BASS being closed source, there is no way to know the exact behaviour that would arise
	/// from passing a pointer the wrong type of struct to this function.
	pub most unsafe fn BASS_SetConfigPtr(option.into(): impl Into<DWORD>, value: *mut c_void) -> bool;
	/// # Safety
	///
	/// Improper casting of the returned pointer to the wrong type of struct will lead to UB.
	///
	/// Consult the docs before (un4seen.com or Cargo docs for this crate) before using this function.
	pub most unsafe fn BASS_GetConfigPtr(option.into(): impl Into<DWORD>) -> *const c_void;
	pub fn BASS_GetVersion() -> DWORD;
	pub fn BASS_ErrorGetCode() -> c_int;
	pub fn BASS_GetDeviceInfo(device.into(): impl Into<DWORD>, info: &mut BASS_DEVICEINFO as *mut BASS_DEVICEINFO) -> bool;
	pub most unsafe fn BASS_Init(
		device: c_int,
		frequency.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>,
		window: *mut c_void,
		dsguid: *mut c_void
	) -> bool;
	pub fn BASS_SetDevice(device.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_GetDevice() -> DWORD;
	pub fn BASS_Free() -> bool;
	pub fn BASS_GetInfo(info: &mut BASS_INFO as *mut BASS_INFO) -> bool;
	pub fn BASS_Update(length.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_GetCPU() -> f32;
	pub fn BASS_Start() -> bool;
	pub fn BASS_Stop() -> bool;
	pub fn BASS_Pause() -> bool;
	pub fn BASS_IsStarted() -> DWORD;
	pub fn BASS_SetVolume(value: f32) -> bool;
	pub fn BASS_GetVolume() -> f32;
	/// When using the `BASS_UNICODE` flag, use a pointer to a UTF-16 CString and simply cast to *const c_char
	pub most unsafe fn BASS_PluginLoad(file: *const c_char, flags.into(): impl Into<DWORD>) -> HPLUGIN;
	pub fn BASS_PluginFree(handle: HPLUGIN) -> bool;
	pub fn BASS_PluginEnable(handle: HPLUGIN, enable.into(): impl Into<BOOL>) -> bool;
	pub fn BASS_PluginGetInfo(handle: HPLUGIN) -> *const BASS_PLUGININFO;
	// 3D
	pub fn BASS_Apply3D();
	pub fn BASS_Set3DFactors(distance: f32, roll: f32, doppler_factor: f32) -> bool;
	pub fn BASS_Get3DFactors(
		distance: &mut Option<f32> as *mut Option<f32> as *mut f32,
		roll: &mut Option<f32> as *mut Option<f32> as *mut f32,
		doppler_factor: &mut Option<f32> as *mut Option<f32> as *mut f32
	) -> bool;
	pub fn BASS_Set3DPosition(
		position: &Option<BASS_3DVECTOR> as *const Option<BASS_3DVECTOR> as *const BASS_3DVECTOR,
		velocity: &Option<BASS_3DVECTOR> as *const Option<BASS_3DVECTOR> as *const BASS_3DVECTOR,
		front: &Option<BASS_3DVECTOR> as *const Option<BASS_3DVECTOR> as *const BASS_3DVECTOR,
		top: &Option<BASS_3DVECTOR> as *const Option<BASS_3DVECTOR> as *const BASS_3DVECTOR
	) -> bool;
	/// # Safety
	///
	/// The parameters of this function are C-style "out" parameters, the recommended access method is to
	/// pass an `&mut Option<BASS_3DVECTOR>` for the fields you wish to access, and `NULL` for those you do not.
	///
	/// To pass "NULL" to this function, pass an `&mut Option::None`.
	pub fn BASS_Get3DPosition(
		position: &mut Option<BASS_3DVECTOR> as *mut Option<BASS_3DVECTOR> as *mut BASS_3DVECTOR,
		velocity: &mut Option<BASS_3DVECTOR> as *mut Option<BASS_3DVECTOR> as *mut BASS_3DVECTOR,
		front: &mut Option<BASS_3DVECTOR> as *mut Option<BASS_3DVECTOR> as *mut BASS_3DVECTOR,
		top: &mut Option<BASS_3DVECTOR> as *mut Option<BASS_3DVECTOR> as *mut BASS_3DVECTOR
	) -> bool;
	pub most unsafe fn BASS_MusicLoad(
		memory.into(): impl Into<BOOL>,
		file: *const c_void,
		offset.into(): impl Into<QWORD>,
		length.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>,
		frequency.into(): impl Into<DWORD>
	) -> HMUSIC;
	pub fn BASS_MusicFree(handle: HMUSIC) -> bool;
	pub most unsafe fn BASS_SampleLoad(
		memory.into(): impl Into<BOOL>,
		file: *const c_void,
		offset.into(): impl Into<QWORD>,
		length.into(): impl Into<DWORD>,
		maximum.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>
	) -> HSAMPLE;
	pub fn BASS_SampleCreate(
		length.into(): impl Into<DWORD>,
		frequency.into(): impl Into<DWORD>,
		channels.into(): impl Into<DWORD>,
		maximum.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>
	) -> HSAMPLE;
	pub fn BASS_SampleFree(handle: HSAMPLE) -> bool;
	/// # Safety
	///
	/// Care must be taken to handle the Vec or Boxed Slice used for the buffer.
	///
	/// Consult the Nomicon for further information.
	pub most unsafe fn BASS_SampleSetData(handle: HSAMPLE, buffer: *const c_void) -> bool;
	/// # Safety
	///
	/// This is marked as unsafe due to the need to manually handle the buffer.
	///
	/// Consult the Nomicon for further information.
	pub most unsafe fn BASS_SampleGetData(handle: HSAMPLE, buffer: *mut c_void) -> bool;
	pub fn BASS_SampleGetInfo(handle: HSAMPLE, info: &mut BASS_SAMPLE as *mut BASS_SAMPLE) -> bool;
	pub fn BASS_SampleSetInfo(handle: HSAMPLE, info: &BASS_SAMPLE as *const BASS_SAMPLE) -> bool;
	pub fn BASS_SampleGetChannel(handle: HSAMPLE, flags.into(): impl Into<DWORD>) -> Option<DWORD>;
	pub most unsafe fn BASS_SampleGetChannels(handle: HSAMPLE, channels: *mut HCHANNEL) -> DWORD;
	pub fn BASS_SampleStop(handle: HSAMPLE) -> bool;
	pub fn BASS_StreamCreate<T>(
		frequency.into(): impl Into<DWORD>,
		channels.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>,
		proc: STREAMPROC,
		user: *mut T as *mut c_void
	) -> HSTREAM;
	pub most unsafe fn BASS_StreamCreateFile(
		memory.into(): impl Into<BOOL>,
		file: *const c_void,
		offset.into(): impl Into<QWORD>,
		length.into(): impl Into<QWORD>,
		flags.into(): impl Into<DWORD>
	) -> HSTREAM;
	pub most unsafe fn BASS_StreamCreateURL<T>(
		url: *const c_char,
		offset.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>,
		proc: DOWNLOADPROC,
		user: *mut T as *mut c_void
	) -> HSTREAM;
	pub fn BASS_StreamCreateFileUser<T>(
		system.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>,
		proc: &BASS_FILEPROCS as *const BASS_FILEPROCS,
		user: *mut T as *mut c_void
	) -> HSTREAM;
	pub fn BASS_StreamFree(handle: HSTREAM) -> bool;
	pub fn BASS_StreamGetFilePosition(handle: HSTREAM, mode.into(): impl Into<DWORD>) -> QWORD;
	pub most unsafe fn BASS_StreamPutData(handle: HSTREAM, buffer: *const c_void, length.into(): impl Into<DWORD>) -> DWORD;
	pub most unsafe fn BASS_StreamPutFileData(handle: HSTREAM, buffer: *const c_void, length.into(): impl Into<DWORD>) -> DWORD;
	pub fn BASS_RecordGetDeviceInfo(device.into(): impl Into<DWORD>, info: &mut BASS_DEVICEINFO as *mut BASS_DEVICEINFO) -> bool;
	pub fn BASS_RecordInit(device: c_int) -> bool;
	pub fn BASS_RecordSetDevice(device.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_RecordGetDevice() -> DWORD;
	pub fn BASS_RecordFree() -> bool;
	pub fn BASS_RecordGetInfo(info: &mut BASS_RECORDINFO as *mut BASS_RECORDINFO) -> bool;
	pub most unsafe fn BASS_RecordGetInputName(input: c_int) -> *const c_char;
	pub fn BASS_RecordSetInput(input: c_int, flags.into(): impl Into<DWORD>, volume: f32) -> bool;
	pub fn BASS_RecordGetInput(input: c_int, volume: &mut f32 as *mut f32) -> DWORD;
	pub fn BASS_RecordStart<T>(
		frequency.into(): impl Into<DWORD>,
		channels.into(): impl Into<DWORD>,
		flags.into(): impl Into<DWORD>,
		proc: RECORDPROC,
		user: *mut T as *mut c_void
	) -> HRECORD;
	pub fn BASS_ChannelBytes2Seconds(handle.into(): impl Into<DWORD>, position.into(): impl Into<QWORD>) -> f64;
	pub fn BASS_ChannelSeconds2Bytes(handle.into(): impl Into<DWORD>, position: f64) -> QWORD;
	pub fn BASS_ChannelGetDevice(handle.into(): impl Into<DWORD>) -> DWORD;
	pub fn BASS_ChannelSetDevice(handle.into(): impl Into<DWORD>, device.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelIsActive(handle.into(): impl Into<DWORD>) -> DWORD;
	pub fn BASS_ChannelGetInfo(handle.into(): impl Into<DWORD>, info: &mut BASS_CHANNELINFO as *mut BASS_CHANNELINFO) -> bool;
	pub most unsafe fn BASS_ChannelGetTags(handle.into(): impl Into<DWORD>, tags.into(): impl Into<DWORD>) -> *const c_char;
	pub fn BASS_ChannelFlags(handle.into(): impl Into<DWORD>, flags.into(): impl Into<DWORD>, mask.into(): impl Into<DWORD>) -> DWORD;
	pub fn BASS_ChannelUpdate(handle.into(): impl Into<DWORD>, length.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelLock(handle.into(): impl Into<DWORD>, lock.into(): impl Into<BOOL>) -> bool;
	pub fn BASS_ChannelFree(handle.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelPlay(handle.into(): impl Into<DWORD>, restart.into(): impl Into<BOOL>) -> bool;
	pub fn BASS_ChannelPause(handle.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelStop(handle.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelStart(handle.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelSetAttribute(handle.into(): impl Into<DWORD>, attribute.into(): impl Into<DWORD>, value: f32) -> bool;
	pub fn BASS_ChannelGetAttribute(handle.into(): impl Into<DWORD>, attribute.into(): impl Into<DWORD>, value: &mut f32 as *mut f32) -> bool;
	pub fn BASS_ChannelSlideAttribute(handle.into(): impl Into<DWORD>, attribute.into(): impl Into<DWORD>, value: f32, time.into(): impl Into<DWORD>)-> bool;
	pub fn BASS_ChannelIsSliding(handle.into(): impl Into<DWORD>, attribute.into(): impl Into<DWORD>) -> bool;
	pub most unsafe fn BASS_ChannelSetAttributeEx(
		handle.into(): impl Into<DWORD>,
		attribute.into(): impl Into<DWORD>,
		value: *mut c_void,
		size.into(): impl Into<DWORD>
	) -> bool;
	pub most unsafe fn BASS_ChannelGetAttributeEx(
		handle.into(): impl Into<DWORD>,
		attribute.into(): impl Into<DWORD>,
		value: *mut c_void,
		size.into(): impl Into<DWORD>
	) -> DWORD;
	pub fn BASS_ChannelSet3DAttributes(
		handle.into(): impl Into<DWORD>,
		mode: c_int,
		minimum: f32,
		maximum: f32,
		iangle: c_int,
		oangle: c_int,
		out_volume: f32
	) -> bool;
	pub fn BASS_ChannelGet3DAttributes(
		handle.into(): impl Into<DWORD>,
		mode: &mut Option<DWORD> as *mut Option<DWORD> as *mut DWORD,
		minimum: &mut Option<f32> as *mut Option<f32> as *mut f32,
		maximum: &mut Option<f32> as *mut Option<f32> as *mut f32,
		angle_of_inside_projection_cone: &mut Option<DWORD> as *mut Option<DWORD> as *mut DWORD,
		angle_of_outside_projection_cone: &mut Option<DWORD> as *mut Option<DWORD> as *mut DWORD,
		output_volume: &mut Option<f32> as *mut Option<f32> as *mut f32
	) -> bool;
	pub fn BASS_ChannelSet3DPosition(
		handle.into(): impl Into<DWORD>,
		position: &Option<BASS_3DVECTOR> as *const Option<BASS_3DVECTOR> as *const BASS_3DVECTOR,
		orientation: &Option<BASS_3DVECTOR> as *const Option<BASS_3DVECTOR> as *const BASS_3DVECTOR,
		velocity: &Option<BASS_3DVECTOR> as *const Option<BASS_3DVECTOR> as *const BASS_3DVECTOR
	) -> bool;
	pub fn BASS_ChannelGet3DPosition(
		handle.into(): impl Into<DWORD>,
		position: &mut Option<BASS_3DVECTOR> as *mut Option<BASS_3DVECTOR> as *mut BASS_3DVECTOR,
		orientation: &mut Option<BASS_3DVECTOR> as *mut Option<BASS_3DVECTOR> as *mut BASS_3DVECTOR,
		velocity: &mut Option<BASS_3DVECTOR> as *mut Option<BASS_3DVECTOR> as *mut BASS_3DVECTOR
	) -> bool;
	pub fn BASS_ChannelGetLength(
		handle.into(): impl Into<DWORD>,
		mode.into(): impl Into<DWORD>
	) -> QWORD;
	pub fn BASS_ChannelSetPosition(
		handle.into(): impl Into<DWORD>,
		position.into(): impl Into<QWORD>,
		mode.into(): impl Into<DWORD>
	) -> bool;
	pub fn BASS_ChannelGetPosition(
		handle.into(): impl Into<DWORD>,
		mode.into(): impl Into<DWORD>
	) -> QWORD;
	pub fn BASS_ChannelGetLevel(handle.into(): impl Into<DWORD>) -> DWORD;
	pub most unsafe fn BASS_ChannelGetLevelEx(
		handle.into(): impl Into<DWORD>,
		levels: *mut f32,
		length: f32,
		flags.into(): impl Into<DWORD>
	) -> bool;
	pub most unsafe fn BASS_ChannelGetData(
		handle.into(): impl Into<DWORD>,
		buffer: *mut c_void,
		length.into(): impl Into<DWORD>
	) -> DWORD;
	/// Set a SYNCPROC on the channel `handle`
	pub fn BASS_ChannelSetSync<T>(
		handle.into(): impl Into<DWORD>,
		sync_type.into(): impl Into<DWORD>,
		parameter.into(): impl Into<QWORD>,
		proc: SYNCPROC,
		user: *mut T as *mut c_void
	) -> HSYNC;
	pub fn BASS_ChannelRemoveSync(handle.into(): impl Into<DWORD>, sync: HSYNC) -> bool;
	pub fn BASS_ChannelSetDSP<T>(
		handle.into(): impl Into<DWORD>,
		proc: DSPPROC,
		user: *mut T as *mut c_void,
		priority: c_int
	) -> HDSP;
	pub fn BASS_ChannelRemoveDSP(handle.into(): impl Into<DWORD>, dsp: HDSP) -> bool;
	pub fn BASS_ChannelSetLink(handle.into(): impl Into<DWORD>, channel.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelRemoveLink(handle.into(): impl Into<DWORD>, channel.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_ChannelSetFX(handle.into(): impl Into<DWORD>, fx_type.into(): impl Into<DWORD>, priority: c_int) -> HFX;
	pub fn BASS_ChannelRemoveFX(handle.into(): impl Into<DWORD>, fx: HFX) -> bool;
	/// # Safety
	///
	/// Utmost care must be taken to ensure memory safety is guaranteed.
	///
	/// Be VERY careful to use the correct structure for an effect.
	pub most unsafe fn BASS_FXSetParameters(handle: HFX, parameters: *const c_void) -> bool;
	/// # Safety
	///
	/// Utmost care must be taken to ensure memory safety is guaranteed.
	///
	/// Be VERY careful to use the correct structure for an effect.
	pub most unsafe fn BASS_FXGetParameters(handle: HFX, parameters: *mut c_void) -> bool;
	pub fn BASS_FXReset(handle.into(): impl Into<DWORD>) -> bool;
	pub fn BASS_FXSetPriority(handle: HFX, priority: c_int) -> bool;
}

// static STATIC_TEST: once_cell::sync::Lazy<libloading::Symbol<extern "system"pub fn(x: u32, y: u32, z: T) -> u32>> = once_cell::sync::Lazy::new(|| {
// 	if let Ok(function) = unsafe { BASS_LIBRARY.get(stringify!("whatever").as_bytes()) } {
// 		return function;
// 	} else {
// 		panic!("Failed to load the function.");
// 	}
// });

// extern "system"pub fn test<T>(x: u32, y: u32, z: T) -> u32 {

// }

// type TestFunc<T> = extern "system"pub fn(x: T) -> u32;
