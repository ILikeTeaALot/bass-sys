// The commented code in this file is for posterity and for others to learn how NOT to do weird function pointers in Rust

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

union FnPointerUnion {
	// proc: STREAMPROC
	proc: unsafe extern "C" fn(
		handle: HSTREAM,
		buffer: *mut ::std::os::raw::c_void,
		length: DWORD,
		user: *mut ::std::os::raw::c_void,
	) -> DWORD,
	_i64: isize,
}

/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
pub const STREAMPROC_DUMMY: STREAMPROC = ::std::option::Option::None;
// static STREAMPROC_DUMMY_CELL: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: 0 }.proc }));
// pub static STREAMPROC_DUMMY: &STREAMPROC = &*STREAMPROC_DUMMY_CELL;
/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
// pub const STREAMPROC_PUSH: STREAMPROC = ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: -1 }.proc });
pub static STREAMPROC_PUSH: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: -1 }.proc }));
/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
// pub const STREAMPROC_DEVICE: STREAMPROC = ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: -2 }.proc });

pub static STREAMPROC_DEVICE: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: -2 }.proc }));
// pub static STREAMPROC_DEVICE: &STREAMPROC = &*STREAMPROC_DEVICE_CELL;
// pub static STREAMPROC_DEVICE: STREAMPROC = ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: -2 }.proc });
/// # Safety
/// 
/// NEVER **EVER** ***EVER*** TRY TO CALL THIS FUNCTION. EVER. IT IS A FAKE POINTER THAT EXISTS BECAUSE OF THE WAY THE BASS LIBRARY WORKS.
// pub const STREAMPROC_DEVICE_3D: STREAMPROC = ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: -3 }.proc });
pub static STREAMPROC_DEVICE_3D: ::once_cell::sync::Lazy<STREAMPROC> = ::once_cell::sync::Lazy::new(|| ::std::option::Option::Some(unsafe { FnPointerUnion { _i64: -3 }.proc }));
