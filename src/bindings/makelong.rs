#![allow(non_snake_case)]

use crate::{BYTE, DWORD, WORD};

/// #define LOBYTE(a) (BYTE)(a)
pub fn LOBYTE(a: DWORD) -> BYTE {
	// a as BYTE // Type Alias Version
	BYTE(*a as u8) // NewType Version
}

/// #define HIBYTE(a) (BYTE)((a)>>8)
pub fn HIBYTE(a: DWORD) -> BYTE {
	// (a >> 8) as BYTE // Type Alias Version
	BYTE((*a >> 8) as u8) // NewType Version
}

/// #define LOWORD(a) (WORD)(a)
pub fn LOWORD(a: DWORD) -> WORD {
	// a as WORD // Type Alias Version
	WORD(*a as u16) // NewType Version
}

/// #define HIWORD(a) (WORD)((a)>>16)
pub fn HIWORD(a: DWORD) -> WORD {
	// (a >> 16) as WORD // Type Alias Version
	WORD((*a >> 16) as u16) // NewType Version
}

/// #define MAKEWORD(a,b) (WORD)(((a)&0xff)|((b)<<8))
pub fn MAKEWORD(a: BYTE, b: BYTE) -> WORD {
	// ((a as WORD) & 0xff) | ((b as WORD) << 8) // Type Alias Version
	WORD(((*a as u16) & 0xff) | ((*b as u16) << 8)) // NewType Version
}

/// #define MAKELONG(a,b) (DWORD)(((a)&0xffff)|((b)<<16))
pub fn MAKELONG(a: WORD, b: WORD) -> DWORD {
	// ((a) & 0xffff) | ((b) << 16u16) // Type Alias Version
	DWORD(((*a) & 0xffff) as u32 | ((*b as u32) << 16)) // NewType Version
}