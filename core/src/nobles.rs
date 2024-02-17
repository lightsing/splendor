use crate::colors::ColorVec;
use serde::Serialize;

/// A struct to represent a noble.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Noble {
    /// The points of the noble.
    pub points: u8,
    /// The color requirements of the noble.
    pub requires: ColorVec,
}

impl Noble {
    /// Define a new noble.
    pub const fn new(points: u8, requires: ColorVec) -> Self {
        Noble { points, requires }
    }
}
