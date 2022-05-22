# byte-enum

A `ByteEnum` derive macro and trait to implement `Into<u8>` and `TryFrom<u8>` for a `#[repr(u8)]` enum. Simple as.

The enum must be `#[repr(u8)]`, fieldless, and may not have explicit discriminants.

----------------------------------------------

```rust
use byte_enum::ByteEnum;

#[derive(ByteEnum)]
#[repr(u8)]
enum SomeEnum {
    VariantA,
    VariantB,
    VariantC,
}

let b: u8 = SomeEnum::VariantB.into();
assert_eq!(b, 1);
 
let c = SomeEnum::try_from(2_u8);
assert_eq!(c, Ok(SomeEnum::VariantC));
 
let d = SomeEnum::try_from(4_u8);
assert!(d.is_err());
```

License
-------

byte-enum may be used under your choice of the Apache 2 or MIT license.
