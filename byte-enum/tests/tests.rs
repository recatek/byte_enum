use byte_enum::*;

#[derive(ByteEnum, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TestEnum {
    VariantA,
    VariantB,
    VariantC,
    VariantD,
}

#[test]
fn test_basic() {
    let x = TestEnum::VariantC;
    let y: u8 = x.into();
    let z: TestEnum = y.try_into().unwrap();
    assert_eq!(x, z);
}
