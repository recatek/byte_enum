pub mod traits;

use traits::*;

use byte_enum_derive::ByteEnum;

#[derive(ByteEnum)]
#[repr(u8)]
enum TestEnum {
    VariantA,
    VariantB,
    VariantC,
    VariantD,
}

#[test]
fn main() {

}
