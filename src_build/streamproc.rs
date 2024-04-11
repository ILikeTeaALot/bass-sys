// pub const STREAMPROC_DUMMY: STREAMPROC = ::std::option::Option::Some(
// 	(0u32 as *const ::std::os::raw::c_void)
// 		as unsafe extern "C" fn(
// 		handle: HSTREAM,
// 		buffer: *mut ::std::os::raw::c_void,
// 		length: DWORD,
// 		user: *mut ::std::os::raw::c_void,
// 	) -> DWORD,
// );
// pub const STREAMPROC_PUSH: STREAMPROC = ::std::option::Option::Some(
// 	(-1i32 as u32 as *const ::std::os::raw::c_void)
// 		as unsafe extern "C" fn(
// 		handle: HSTREAM,
// 		buffer: *mut ::std::os::raw::c_void,
// 		length: DWORD,
// 		user: *mut ::std::os::raw::c_void,
// 	) -> DWORD,
// );
// pub const STREAMPROC_DEVICE: STREAMPROC = ::std::option::Option::Some(
// 	(-2i32 as u32 as *const ::std::os::raw::c_void)
// 		as unsafe extern "C" fn(
// 		handle: HSTREAM,
// 		buffer: *mut ::std::os::raw::c_void,
// 		length: DWORD,
// 		user: *mut ::std::os::raw::c_void,
// 	) -> DWORD,
// );
// pub const STREAMPROC_DEVICE_3D: STREAMPROC = ::std::option::Option::Some(
// 	(-3i32 as u32 as *const ::std::os::raw::c_void)
// 		as unsafe extern "C" fn(
// 			handle: HSTREAM,
// 			buffer: *mut ::std::os::raw::c_void,
// 			length: DWORD,
// 			user: *mut ::std::os::raw::c_void,
// 		) -> DWORD,
// );

union FunnyFnPointer {
	// proc: STREAMPROC
	proc: unsafe extern "C" fn(
		handle: HSTREAM,
		buffer: *mut ::std::os::raw::c_void,
		length: DWORD,
		user: *mut ::std::os::raw::c_void,
	) -> DWORD,
	_i64: i64,
}

/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
// pub const STREAMPROC_DUMMY: STREAMPROC = ::std::option::Option::None;
pub const STREAMPROC_DUMMY: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FunnyFnPointer { _i64: 0 }.proc }));
/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
// pub const STREAMPROC_PUSH: STREAMPROC = ::std::option::Option::Some(unsafe { FunnyFnPointer { _i64: -1 }.proc });
pub static STREAMPROC_PUSH: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FunnyFnPointer { _i64: -1 }.proc }));
/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
// pub const STREAMPROC_DEVICE: STREAMPROC = ::std::option::Option::Some(unsafe { FunnyFnPointer { _i64: -2 }.proc });
pub const STREAMPROC_DEVICE: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FunnyFnPointer { _i64: -2 }.proc }));
/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
// pub const STREAMPROC_DEVICE_3D: STREAMPROC = ::std::option::Option::Some(unsafe { FunnyFnPointer { _i64: -3 }.proc });
pub const STREAMPROC_DEVICE_3D: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FunnyFnPointer { _i64: -3 }.proc }));
