use once_cell::sync::Lazy;

use crate::{
    bindings::*, generate_bindings
};
use std::os::raw::{c_int, c_void};
use crate::dword::AsDWORD;

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
    fn BASS_CD_SetInterface(iface: DWORD) -> DWORD;
    fn BASS_CD_GetInfo(drive: DWORD, info: *mut BASS_CD_INFO) -> BOOL;
    fn BASS_CD_Door(drive: DWORD, action: DWORD) -> BOOL;
    fn BASS_CD_DoorIsOpen(drive: DWORD) -> BOOL;
    fn BASS_CD_DoorIsLocked(drive: DWORD) -> BOOL;
    fn BASS_CD_IsReady(drive: DWORD) -> BOOL;
    fn BASS_CD_GetTracks(drive: DWORD) -> DWORD;
    fn BASS_CD_GetTrackLength(drive: DWORD, track: DWORD) -> DWORD;
    fn BASS_CD_GetTrackPregap(drive: DWORD, track: DWORD) -> DWORD;
    fn BASS_CD_GetTOC(drive: DWORD, mode: DWORD, toc: *mut BASS_CD_TOC) -> BOOL;
    fn BASS_CD_GetID(drive: DWORD, id: DWORD) -> *const ::std::os::raw::c_char;
    fn BASS_CD_GetSpeed(drive: DWORD) -> DWORD;
    fn BASS_CD_SetSpeed(drive: DWORD, speed: DWORD) -> BOOL;
    fn BASS_CD_GetCache(drive: DWORD) -> DWORD;
    fn BASS_CD_SetCache(drive: DWORD, enable: BOOL) -> BOOL;
    fn BASS_CD_SetOffset(drive: DWORD, offset: ::std::os::raw::c_int) -> BOOL;
    fn BASS_CD_Release(drive: DWORD) -> BOOL;
    fn BASS_CD_StreamCreate(drive: DWORD, track: DWORD, flags: DWORD) -> HSTREAM;
    fn BASS_CD_StreamCreateFile(file: *const ::std::os::raw::c_char, flags: DWORD) -> HSTREAM;
    fn BASS_CD_StreamCreateEx(
        drive: DWORD,
        track: DWORD,
        flags: DWORD,
        proc_: CDDATAPROC,
        user: *mut ::std::os::raw::c_void,
    ) -> HSTREAM;
    fn BASS_CD_StreamCreateFileEx(
        file: *const ::std::os::raw::c_char,
        flags: DWORD,
        proc_: CDDATAPROC,
        user: *mut ::std::os::raw::c_void,
    ) -> HSTREAM;
    fn BASS_CD_StreamGetTrack(handle: HSTREAM) -> DWORD;
    fn BASS_CD_StreamSetTrack(handle: HSTREAM, track: DWORD) -> BOOL;
    fn BASS_CD_Analog_Play(drive: DWORD, track: DWORD, pos: DWORD) -> BOOL;
    fn BASS_CD_Analog_PlayFile(file: *const ::std::os::raw::c_char, pos: DWORD) -> DWORD;
    fn BASS_CD_Analog_Stop(drive: DWORD) -> BOOL;
    fn BASS_CD_Analog_IsActive(drive: DWORD) -> DWORD;
    fn BASS_CD_Analog_GetPosition(drive: DWORD) -> DWORD;
}