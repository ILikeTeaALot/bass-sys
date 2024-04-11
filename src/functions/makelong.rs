use crate::{BYTE, DWORD, WORD};

/// #define LOBYTE(a) (BYTE)(a)
pub fn LOBYTE(a: DWORD) -> BYTE {
	a as BYTE
}

/// #define HIBYTE(a) (BYTE)((a)>>8)
pub fn HIBYTE(a: DWORD) -> BYTE {
	(a >> 8) as BYTE
}

/// #define LOWORD(a) (WORD)(a)
pub fn LOWORD(a: DWORD) -> WORD {
	a as WORD
}

/// #define HIWORD(a) (WORD)((a)>>16)
pub fn HIWORD(a: DWORD) -> WORD {
	(a >> 16) as WORD
}

/// #define MAKEWORD(a,b) (WORD)(((a)&0xff)|((b)<<8))
pub fn MAKEWORD(a: BYTE, b: BYTE) -> WORD {
	((a as WORD) & 0xff) | ((b as WORD) << 8)
}

/// #define MAKELONG(a,b) (DWORD)(((a)&0xffff)|((b)<<16))
pub fn MAKELONG(a: DWORD, b: DWORD) -> DWORD {
	((a) & 0xffff) | ((b) << 16u16)
}