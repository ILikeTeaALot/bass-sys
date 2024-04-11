use std::ops::{Deref, Not};

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BOOL {
	FALSE = 0,
	TRUE = 1,
}

impl Default for BOOL {
	fn default() -> Self {
		Self::FALSE
	}
}

impl Not for BOOL {
	type Output = bool;

	fn not(self) -> Self::Output {
		!match self {
			BOOL::FALSE => false,
			_ => true,
		}
	}
}

impl AsRef<bool> for BOOL {
	fn as_ref(&self) -> &bool {
		&self
	}
}

// impl Into<bool> for BOOL {
// 	fn into(self) -> bool {
// 		match self {
// 			BOOL::FALSE => false,
// 			_ => true,
// 		}
// 	}
// }

impl From<BOOL> for bool {
	fn from(value: BOOL) -> Self {
		match value {
			BOOL::FALSE => false,
			_ => true,
		}
	}
}

impl Deref for BOOL {
	type Target = bool;

	fn deref(&self) -> &Self::Target {
		match self {
			BOOL::FALSE => &false,
			_ => &true,
		}
	}
}

impl From<BOOL> for DWORD {
	fn from(value: BOOL) -> DWORD {
		// DWORD(value as u32)
		value as u32
	}
}

// impl From<BOOL> for u32 {
// 	fn from(value: BOOL) -> Self {
// 		value as u32
// 	}
// }

pub const TRUE: BOOL = BOOL::TRUE;
pub const FALSE: BOOL = BOOL::FALSE;