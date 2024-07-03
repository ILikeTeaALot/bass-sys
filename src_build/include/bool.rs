use std::ops::{Deref, Not};

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BOOL {
	FALSE = 0,
	TRUE = 1,
}

impl BOOL {
	pub fn ok(&self) -> bool {
		self.is_true()
	}

	pub fn is_true(&self) -> bool {
		*self == BOOL::TRUE
	}

	pub fn is_false(&self) -> bool {
		*self == BOOL::FALSE
	}
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

impl From<BOOL> for bool {
	fn from(value: BOOL) -> Self {
		match value {
			BOOL::FALSE => false,
			_ => true,
		}
	}
}

impl From<bool> for BOOL {
	fn from(value: bool) -> Self {
		if value {
			BOOL::TRUE
		} else {
			BOOL::FALSE
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
		DWORD(value as u32) // NewType
		// value as u32 // Alias
	}
}

pub const TRUE: BOOL = BOOL::TRUE;
pub const FALSE: BOOL = BOOL::FALSE;