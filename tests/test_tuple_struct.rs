extern crate struct_deser;
#[macro_use]
extern crate struct_deser_derive;

#[derive(StructDeser, Debug, Eq, PartialEq)]
#[struct_deser(identifier = "47", identifier_type = "u8")]
struct Integers(
    u8,
    i8,
    #[be] u16,
    #[be] i16,
    #[le] u16,
    #[le] i16,
    #[be] u32,
    #[be] i32,
    #[le] u32,
    #[le] i32,
    #[be] u64,
    #[be] i64,
    #[le] u64,
    #[le] i64,
);

#[test]
fn main() {
    use struct_deser::{FromBytes, Identifier, IntoBytes, SerializedByteLen};

    assert_eq!(Integers::IDENTIFIER, 47);
    assert_eq!(Integers::BYTE_LEN, 58);

    let integers = Integers(42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55);

    let mut bytes = [0; Integers::BYTE_LEN];
    integers.into_bytes(&mut bytes);
    let integers2 = Integers::from_bytes(&bytes);

    assert_eq!(integers, integers2);
}
