use crate::colors::ColorVec;
use serde::{Deserialize, Serialize};

/// A struct to represent a noble.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Noble {
    /// The color requirements of the noble.
    pub requires: ColorVec,
}

impl Noble {
    /// Define a new noble.
    pub const fn new(requires: ColorVec) -> Self {
        Noble { requires }
    }
}
