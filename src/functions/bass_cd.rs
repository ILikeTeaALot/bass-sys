use once_cell::sync::Lazy;

use crate::{
    bindings::*, generate_bindings
};
use std::os::raw::{c_int, c_void};

static BASS_CD_LIBRARY: Lazy<BASS_CD> = Lazy::new(|| {
    #[cfg(target_os = "windows")]
        let library_name = "basscd.dll";
    #[cfg(target_os = "linux")]
        let library_name = "libbasscd.so";
    if let Ok(library) = unsafe { BASS_CD::new(library_name) } {
        library
    } else {
        panic!("Failed to load the library.");
    }
});

generate_bindings! {
	BASS_CD_LIBRARY;
	"/doc/basscd/";
    pub fn BASS_CD_SetInterface(iface.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_CD_GetInfo(drive.into(): impl Into<DWORD>, info: &mut BASS_CD_INFO as *mut BASS_CD_INFO) -> bool;
    pub fn BASS_CD_Door(drive.into(): impl Into<DWORD>, action.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_DoorIsOpen(drive.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_DoorIsLocked(drive.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_IsReady(drive.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_GetTracks(drive.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_CD_GetTrackLength(drive.into(): impl Into<DWORD>, track.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_CD_GetTrackPregap(drive.into(): impl Into<DWORD>, track.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_CD_GetTOC(drive.into(): impl Into<DWORD>, mode.into(): impl Into<DWORD>, toc: &mut BASS_CD_TOC as *mut BASS_CD_TOC) -> bool;
    pub fn BASS_CD_GetID(drive.into(): impl Into<DWORD>, id.into(): impl Into<DWORD>) -> *const ::std::os::raw::c_char;
    pub fn BASS_CD_GetSpeed(drive.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_CD_SetSpeed(drive.into(): impl Into<DWORD>, speed.into(): impl Into<DWORD>) -> bool;
    // pub fn BASS_CD_GetCache(drive.into(): impl Into<DWORD>) -> DWORD;
    // pub fn BASS_CD_SetCache(drive.into(): impl Into<DWORD>, enable: BOOL) -> bool;
    pub fn BASS_CD_SetOffset(drive.into(): impl Into<DWORD>, offset: ::std::os::raw::c_int) -> bool;
    pub fn BASS_CD_Release(drive.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_StreamCreate(drive.into(): impl Into<DWORD>, track.into(): impl Into<DWORD>, flags.into(): impl Into<DWORD>) -> HSTREAM;
    pub fn BASS_CD_StreamCreateFile(file: *const ::std::os::raw::c_char, flags.into(): impl Into<DWORD>) -> HSTREAM;
    pub fn BASS_CD_StreamCreateEx(
        drive.into(): impl Into<DWORD>,
        track.into(): impl Into<DWORD>,
        flags.into(): impl Into<DWORD>,
        proc: CDDATAPROC,
        user: *mut ::std::os::raw::c_void,
    ) -> HSTREAM;
    pub fn BASS_CD_StreamCreateFileEx(
        file: *const ::std::os::raw::c_char,
        flags.into(): impl Into<DWORD>,
        proc: CDDATAPROC,
        user: *mut ::std::os::raw::c_void,
    ) -> HSTREAM;
    pub fn BASS_CD_StreamGetTrack(handle: HSTREAM) -> DWORD;
    pub fn BASS_CD_StreamSetTrack(handle: HSTREAM, track.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_Analog_Play(drive.into(): impl Into<DWORD>, track.into(): impl Into<DWORD>, pos.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_Analog_PlayFile(file: *const ::std::os::raw::c_char, pos.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_CD_Analog_Stop(drive.into(): impl Into<DWORD>) -> bool;
    pub fn BASS_CD_Analog_IsActive(drive.into(): impl Into<DWORD>) -> DWORD;
    pub fn BASS_CD_Analog_GetPosition(drive.into(): impl Into<DWORD>) -> DWORD;
}