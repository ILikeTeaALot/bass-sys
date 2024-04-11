use once_cell::sync::Lazy;

use crate::{
	generate_bindings,
	// types::{
	//     Bass3DVector, BassChannelInfo, BassDeviceInfo, BassFileProcs, BassInfo, BassPluginInfo,
	//     BassRecordInfo, BassSample, BOOL, DOWNLOADPROC, DSPPROC, DWORD, HCHANNEL, HDSP, HFX,
	//     HMUSIC, HPLUGIN, HRECORD, HSAMPLE, HSTREAM, HSYNC, QWORD, RECORDPROC, STREAMPROC, SYNCPROC,
	// },
	bindings::*,
};
use std::os::raw::{c_char, c_int, c_void};
use crate::dword::AsDWORD;

static BASS_LIBRARY: Lazy<BASS> = Lazy::new(|| {
	#[cfg(target_os = "macos")]
		let library_name = "libbass.dylib";
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
// pub union AsDWORD {
//     u32: u32,
//     i32: i32,
//     DWORD: DWORD,
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
    fn BASS_SetConfig(option: DWORD, value.to(): impl AsDWORD) -> BOOL;
    fn BASS_GetConfig(option: DWORD) -> DWORD;
    fn BASS_SetConfigPtr(option: DWORD, value: *mut c_void) -> BOOL;
    fn BASS_GetConfigPtr(option: DWORD) -> *const c_void;
    fn BASS_GetVersion() -> DWORD;
    fn BASS_ErrorGetCode() -> c_int;
    fn BASS_GetDeviceInfo(device: DWORD, info: *mut BASS_DEVICEINFO) -> BOOL;
    fn BASS_Init(
        device: c_int,
        frequency: DWORD,
        flags: DWORD,
        window: *mut c_void,
        dsguid: *mut c_void
    ) -> BOOL;
    fn BASS_SetDevice(device: DWORD) -> BOOL;
    fn BASS_GetDevice() -> DWORD;
    fn BASS_Free() -> BOOL;
    fn BASS_GetInfo(info: *mut BASS_INFO) -> BOOL;
    fn BASS_Update(length: DWORD) -> BOOL;
    fn BASS_GetCPU() -> f32;
    fn BASS_Start() -> BOOL;
    fn BASS_Stop() -> BOOL;
    fn BASS_Pause() -> BOOL;
    fn BASS_IsStarted() -> DWORD;
    fn BASS_SetVolume(value: f32) -> BOOL;
    fn BASS_GetVolume() -> f32;
    /// When using the `BASS_UNICODE` flag, use a pointer to a UTF-16 CString and simply cast to *const c_char
    fn BASS_PluginLoad(file: *const c_char, flags: DWORD) -> HPLUGIN;
    fn BASS_PluginFree(handle: HPLUGIN) -> BOOL;
    fn BASS_PluginEnable(handle: HPLUGIN, enable: BOOL) -> BOOL;
    fn BASS_PluginGetInfo(handle: HPLUGIN) -> *const BASS_PLUGININFO;
    fn BASS_Set3DFactors(distance: f32, roll: f32, doppler_factor: f32) -> BOOL;
    fn BASS_Get3DFactors(distance: *mut f32, roll: *mut f32, doppler_factor: *mut f32) -> BOOL;
    fn BASS_Set3DPosition(
        position: *const BASS_3DVECTOR,
        velocity: *const BASS_3DVECTOR,
        front: *const BASS_3DVECTOR,
        top: *const BASS_3DVECTOR
    ) -> BOOL;
    fn BASS_Get3DPosition(
        position: *mut BASS_3DVECTOR,
        velocity: *mut BASS_3DVECTOR,
        front: *mut BASS_3DVECTOR,
        top: *mut BASS_3DVECTOR
    ) -> BOOL;
    fn BASS_Apply3D();
    fn BASS_MusicLoad(
        memory: BOOL,
        file: *const c_void,
        offset: QWORD,
        length: DWORD,
        flags: DWORD,
        frequency: DWORD
    ) -> HMUSIC;
    fn BASS_MusicFree(handle: HMUSIC) -> BOOL;
    fn BASS_SampleLoad(
        memory: BOOL,
        file: *const c_void,
        offset: QWORD,
        length: DWORD,
        maximum: DWORD,
        flags: DWORD
    ) -> HSAMPLE;
    fn BASS_SampleCreate(
        length: DWORD,
        frequency: DWORD,
        channels: DWORD,
        maximum: DWORD,
        flags: DWORD
    ) -> HSAMPLE;
    fn BASS_SampleFree(handle: HSAMPLE) -> BOOL;
    fn BASS_SampleSetData(handle: HSAMPLE, buffer: *const c_void) -> BOOL;
    fn BASS_SampleGetData(handle: HSAMPLE, buffer: *mut c_void) -> BOOL;
    fn BASS_SampleGetInfo(handle: HSAMPLE, info: *mut BASS_SAMPLE) -> BOOL;
    fn BASS_SampleSetInfo(handle: HSAMPLE, info: *const BASS_SAMPLE) -> BOOL;
    fn BASS_SampleGetChannel(handle: HSAMPLE, flags: DWORD) -> DWORD;
    fn BASS_SampleGetChannels(handle: HSAMPLE, channels: *mut HCHANNEL) -> DWORD;
    fn BASS_SampleStop(handle: HSAMPLE) -> BOOL;
    fn BASS_StreamCreate<T>(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: STREAMPROC,
        user: *mut T as *mut c_void
    ) -> HSTREAM;
    fn BASS_StreamCreateFile(
        memory: BOOL,
        file: *const c_void,
        offset: QWORD,
        length: QWORD,
        flags: DWORD
    ) -> HSTREAM;
    fn BASS_StreamCreateURL<T>(
        url: *const c_char,
        offset: DWORD,
        flags: DWORD,
        proc: DOWNLOADPROC,
        user: *mut T as *mut c_void
    ) -> HSTREAM;
    fn BASS_StreamCreateFileUser<T>(
        system: DWORD,
        flags: DWORD,
        proc: *const BASS_FILEPROCS,
        user: *mut T as *mut c_void
    ) -> HSTREAM;
    fn BASS_StreamFree(handle: HSTREAM) -> BOOL;
    fn BASS_StreamGetFilePosition(handle: HSTREAM, mode: DWORD) -> QWORD;
    fn BASS_StreamPutData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    fn BASS_StreamPutFileData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    fn BASS_RecordGetDeviceInfo(device: DWORD, info: *mut BASS_DEVICEINFO) -> BOOL;
    fn BASS_RecordInit(device: c_int) -> BOOL;
    fn BASS_RecordSetDevice(device: DWORD) -> BOOL;
    fn BASS_RecordGetDevice() -> DWORD;
    fn BASS_RecordFree() -> BOOL;
    fn BASS_RecordGetInfo(info: *mut BASS_RECORDINFO) -> BOOL;
    fn BASS_RecordGetInputName(input: c_int) -> *const c_char;
    fn BASS_RecordSetInput(input: c_int, flags: DWORD, volume: f32) -> BOOL;
    fn BASS_RecordGetInput(input: c_int, volume: *mut f32) -> DWORD;
    fn BASS_RecordStart<T>(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: RECORDPROC,
        user: *mut T as *mut c_void
    ) -> HRECORD;
    fn BASS_ChannelBytes2Seconds(handle.to(): impl AsDWORD, position: QWORD) -> f64;
    fn BASS_ChannelSeconds2Bytes(handle.to(): impl AsDWORD, position: f64) -> QWORD;
    fn BASS_ChannelGetDevice(handle.to(): impl AsDWORD) -> DWORD;
    fn BASS_ChannelSetDevice(handle.to(): impl AsDWORD, device: DWORD) -> BOOL;
    fn BASS_ChannelIsActive(handle.to(): impl AsDWORD) -> DWORD;
    fn BASS_ChannelGetInfo(handle.to(): impl AsDWORD, info: *mut BASS_CHANNELINFO) -> BOOL;
    fn BASS_ChannelGetTags(handle.to(): impl AsDWORD, tags: DWORD) -> *const c_char;
    fn BASS_ChannelFlags(handle.to(): impl AsDWORD, flags: DWORD, mask: DWORD) -> DWORD;
    fn BASS_ChannelUpdate(handle.to(): impl AsDWORD, length: DWORD) -> BOOL;
    fn BASS_ChannelLock(handle.to(): impl AsDWORD, lock: BOOL) -> BOOL;
    fn BASS_ChannelFree(handle.to(): impl AsDWORD) -> BOOL;
    fn BASS_ChannelPlay(handle.to(): impl AsDWORD, restart: BOOL) -> BOOL;
    fn BASS_ChannelStop(handle.to(): impl AsDWORD) -> BOOL;
    fn BASS_ChannelPause(handle.to(): impl AsDWORD) -> BOOL;
    fn BASS_ChannelSetAttribute(handle.to(): impl AsDWORD, attribute: DWORD, value: f32) -> BOOL;
    fn BASS_ChannelGetAttribute(handle.to(): impl AsDWORD, attribute: DWORD, value: *mut f32) -> BOOL;
    fn BASS_ChannelSlideAttribute(handle.to(): impl AsDWORD, attribute: DWORD, value: f32, time: DWORD)-> BOOL;
    fn BASS_ChannelIsSliding(handle.to(): impl AsDWORD, attribute: DWORD) -> BOOL;
    fn BASS_ChannelSetAttributeEx(
        handle.to(): impl AsDWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD
    ) -> BOOL;
    fn BASS_ChannelGetAttributeEx(
        handle.to(): impl AsDWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD
    ) -> DWORD;
    fn BASS_ChannelSet3DAttributes(
        handle.to(): impl AsDWORD,
        mode: c_int,
        minimum: f32,
        maximum: f32,
        iangle: c_int,
        oangle: c_int,
        out_volume: f32
    ) -> BOOL;
    fn BASS_ChannelGet3DAttributes(
        handle.to(): impl AsDWORD,
        mode: *mut DWORD,
        minimum: *mut f32,
        maximum: *mut f32,
        angle_of_inside_projection_cone: *mut DWORD,
        angle_of_outside_projection_cone: *mut DWORD,
        output_volume: *mut f32
    ) -> BOOL;
    fn BASS_ChannelSet3DPosition(
        handle.to(): impl AsDWORD,
        position: *const BASS_3DVECTOR,
        orientation: *const BASS_3DVECTOR,
        velocity: *const BASS_3DVECTOR
    ) -> BOOL;
    fn BASS_ChannelGet3DPosition(
        handle.to(): impl AsDWORD,
        position: *mut BASS_3DVECTOR,
        orientation: *mut BASS_3DVECTOR,
        velocity: *mut BASS_3DVECTOR
    ) -> BOOL;
    fn BASS_ChannelGetLength(handle.to(): impl AsDWORD, mode: DWORD) -> QWORD;
    fn BASS_ChannelSetPosition(handle.to(): impl AsDWORD, position: QWORD, mode: DWORD) -> BOOL;
    fn BASS_ChannelGetPosition(handle.to(): impl AsDWORD, mode: DWORD) -> QWORD;
    fn BASS_ChannelGetLevel(handle.to(): impl AsDWORD) -> DWORD;
    fn BASS_ChannelGetLevelEx(handle.to(): impl AsDWORD, levels: *mut f32, length: f32, flags: DWORD) -> BOOL;
    fn BASS_ChannelGetData(handle.to(): impl AsDWORD, buffer: *mut c_void, length: DWORD) -> DWORD;
    /// Set a SYNCPROC on the channel `handle`
    fn BASS_ChannelSetSync<T>(
        handle.to(): impl AsDWORD,
        sync_type: DWORD,
        parameter: QWORD,
        proc: SYNCPROC,
        user: *mut T as *mut c_void
    ) -> HSYNC;
    fn BASS_ChannelRemoveSync(handle.to(): impl AsDWORD, sync: HSYNC) -> BOOL;
    fn BASS_ChannelSetDSP<T>(handle.to(): impl AsDWORD, proc: DSPPROC, user: *mut T as *mut c_void, priority: c_int) -> HDSP;
    fn BASS_ChannelRemoveDSP(handle.to(): impl AsDWORD, dsp: HDSP) -> BOOL;
    fn BASS_ChannelSetLink(handle.to(): impl AsDWORD, channel: DWORD) -> BOOL;
    fn BASS_ChannelRemoveLink(handle.to(): impl AsDWORD, channel: DWORD) -> BOOL;
    fn BASS_ChannelSetFX(handle.to(): impl AsDWORD, fx_type: DWORD, priority: c_int) -> HFX;
    fn BASS_ChannelRemoveFX(handle.to(): impl AsDWORD, fx: HFX) -> BOOL;
    fn BASS_FXSetParameters(handle: HFX, parameters: *const c_void) -> BOOL;
    fn BASS_FXGetParameters(handle: HFX, parameters: *mut c_void) -> BOOL;
    fn BASS_FXReset(handle.to(): impl AsDWORD) -> BOOL;
    fn BASS_FXSetPriority(handle: HFX, priority: c_int) -> BOOL;
}

// static STATIC_TEST: once_cell::sync::Lazy<libloading::Symbol<extern "system" fn(x: u32, y: u32, z: T) -> u32>> = once_cell::sync::Lazy::new(|| {
// 	if let Ok(function) = unsafe { BASS_LIBRARY.get(stringify!("whatever").as_bytes()) } {
// 		return function;
// 	} else {
// 		panic!("Failed to load the function.");
// 	}
// });

// extern "system" fn test<T>(x: u32, y: u32, z: T) -> u32 {

// }

// type TestFunc<T> = extern "system" fn(x: T) -> u32;