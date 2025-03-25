use once_cell::sync::Lazy;

use crate::{bindings::*, generate_bindings};

static BASS_LOUD_LIBRARY: Lazy<BASS_Loud> = Lazy::new(|| {
	#[cfg(target_os = "macos")]
	let library_name = "@executable_path/libbassloud.dylib";
	#[cfg(target_os = "windows")]
	let library_name = "bassloud.dll";
	#[cfg(target_os = "linux")]
	let library_name = "libbassloud.so";
	if let Ok(library) = unsafe { BASS_Loud::new(library_name) } {
		library
	} else {
		panic!("Failed to load the library.");
	}
});

generate_bindings! {
	BASS_LOUD_LIBRARY;
	"/doc/bassloud/";
	// Info
	pub fn BASS_Loudness_GetVersion() -> DWORD;
	// Measurement

	// Retrieves the channel that a loudness measurement is set on.
	// ```h
	// DWORD BASS_Loudness_GetChannel(
	//     HLOUDNESS handle
	// );
	// ```
	// Parameters
	//
	// handle	The loudness measurement.
	//
	// Return value
	//
	// If successful, the loudness measurement's channel handle is returned, else 0 is returned. Use BASS_ErrorGetCode to get the error code.
	//
	// Error codes
	//
	// BASS_ERROR_HANDLE	handle is not valid.
	//
	// See also
	//
	// BASS_Loudness_SetChannel, BASS_Loudness_Start
	pub fn BASS_Loudness_GetChannel(handle: HLOUDNESS) -> DWORD;
	// Retrieves the level of a loudness measurement.
	//
	// ```c
	// BOOL BASS_Loudness_GetLevel(
	//     HLOUDNESS handle,
	//     DWORD mode,
	//     float *level
	// );
	// ```
	// ## Parameters
	//
	// |Param|Mode/Desc|Mode Desc|
	// |-----|---------|---------|
	// |handle	|The loudness measurement handle.|
	// |mode	|The measurement type to retrieve. One of the following.|
	//
	// |	|BASS_LOUDNESS_CURRENT | Loudness in LUFS of the last 400ms or the duration (in milliseconds) specified in the HIWORD; use MAKELONG(mode,duration).
	// |	|BASS_LOUDNESS_INTEGRATED | Integrated loudness in LUFS. This is the average since measurement started.
	// |	|BASS_LOUDNESS_RANGE | Loudness range in LU.
	// |	|BASS_LOUDNESS_PEAK	Peak | level in linear scale.
	// |	|BASS_LOUDNESS_TRUEPEAK | True peak level in linear scale.
	// |level	||Pointer to a variable to receive the measurement level.
	//
	// ## Return value
	//
	// If successful, TRUE is returned, else FALSE is returned. Use BASS_ErrorGetCode to get the error code.
	//
	// ## Remarks
	//
	// Loudness is measured according to the ITU-R BS.1770-4 standard (LUFS = LKFS). The BASS_LOUDNESS_INTEGRATED measurement is gated so that much quieter periods are excluded and do not bring down the measured level, while the other measurements are not gated. Loudness (not peak) levels will be -infinity in the case of pure silence, or if the minimum amount of data has not been processed yet. The BASS_LOUDNESS_INTEGRATED measurement requires at least 400ms of data and is updated every 100ms after that. The BASS_LOUDNESS_RANGE measurement requires at least 4s of data and is updated every 1s after that. The other measurements are updated with every sample, but the BASS_LOUDNESS_CURRENT levels may be lower than expected before the requested duration has been processed.
	// The BASS_LOUDNESS_CURRENT mode can be used with durations of 400ms and 3000ms to get EBU R128 "momentary" and "short-term" loudness levels, respectively. The other modes are equivalent to the EBU R128 measurements of the same name.
	//
	// ## Error codes
	//
	// BASS_ERROR_HANDLE	handle is not valid.
	// BASS_ERROR_ILLPARAM	mode is not valid. If requesting a duration with BASS_LOUDNESS_CURRENT then it exceeds what has been enabled.
	// BASS_ERROR_NOTAVAIL	The requested measurement has not been enabled.
	//
	// ## Example
	//
	// Get the loudness level of the last 1 second.
	// ```c
	// float level;
	// BASS_Loudness_GetLevel(handle, MAKELONG(BASS_LOUDNESS_CURRENT, 1000), &level);
	// ```
	// Get the true peak level in decibels.
	// ```c
	// float level;
	// BASS_Loudness_GetLevel(handle, BASS_LOUDNESS_TRUEPEAK, &level); // get true peak level
	// float dblevel = log10(level) * 20; // translate it to dB
	// ```
	//
	// ## See also
	//
	// BASS_Loudness_GetLevelMulti, BASS_Loudness_Start
	pub fn BASS_Loudness_GetLevel(handle: HLOUDNESS, mode.into(): impl Into<DWORD>, level: &mut f32 as *mut f32) -> bool;
}