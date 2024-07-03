use crate::WORD;

impl From<u16> for WORD {
    fn from(value: u16) -> Self {
        WORD(value)
    }
}

impl From<i16> for WORD {
    fn from(value: i16) -> Self {
        WORD(value as u16)
    }
}

/// Due to Rust's default assumption that all numbers are i32 unless otherwise specified,
/// this implementation exists to ease the use of BASS structs.
impl From<i32> for WORD {
    fn from(value: i32) -> Self {
        WORD(value as u16)
    }
}