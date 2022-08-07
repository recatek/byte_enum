use core::fmt;

pub mod traits {
    /// A simple marker trait automatically added to `#[derive(ByteEnum)]` enums.
    pub trait ByteEnum: Into<u8> + TryFrom<u8> {}
}

pub mod derives {
    pub use byte_enum_derive::ByteEnum;
}

pub use derives::*;
pub use traits::*;

/// Error returned by a failed conversion from `u8` to a `ByteEnum`.
///
/// Contains the attempted value that failed to match to a discriminant.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryEnumFromByteError(pub u8);

impl fmt::Display for TryEnumFromByteError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to match value {} to enum discriminant", self.0)
    }
}

impl std::error::Error for TryEnumFromByteError {}
