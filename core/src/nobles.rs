use crate::colors::ColorVec;
use abi_stable::StableAbi;

/// A struct to represent a noble.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct Noble {
    /// The points of the noble.
    pub points: u8,
    /// The color requirements of the noble.
    pub requires: ColorVec,
}

impl Noble {
    pub const fn new(points: u8, requires: ColorVec) -> Self {
        Noble { points, requires }
    }
}
