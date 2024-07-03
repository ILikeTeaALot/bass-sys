use crate::BYTE;

impl From<u8> for BYTE {
    fn from(value: u8) -> Self {
        BYTE(value)
    }
}

impl From<i8> for BYTE {
    fn from(value: i8) -> Self {
        BYTE(value as u8)
    }
}

/// Due to Rust's default assumption that all numbers are i32 unless otherwise specified,
/// this implementation exists to ease the use of BASS structs.
impl From<i32> for BYTE {
    fn from(value: i32) -> Self {
        BYTE(value as u8)
    }
}