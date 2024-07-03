use crate::QWORD;

impl From<u64> for QWORD {
    fn from(value: u64) -> Self {
        QWORD(value)
    }
}

impl From<i64> for QWORD {
    fn from(value: i64) -> Self {
        QWORD(value as u64)
    }
}

/// Due to Rust's default assumption that all numbers are i32 unless otherwise specified,
/// this implementation exists to ease the use of BASS functions that require QWORD parameters.
impl From<i32> for QWORD {
    fn from(value: i32) -> Self {
        QWORD(value as u64)
    }
}

impl From<usize> for QWORD {
    fn from(value: usize) -> Self {
        QWORD(value as u64)
    }
}