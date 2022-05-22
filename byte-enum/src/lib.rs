pub mod traits {
    pub trait IsByteEnum: Into<u8> + TryFrom<u8> {}
}

pub mod derives {
    pub use byte_enum_derive::ByteEnum;
}

pub use derives::*;
pub use traits::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryEnumFromByteError(pub u8);
