extern crate struct_deser;
#[macro_use]
extern crate struct_deser_derive;

#[derive(StructDeser, Debug, Eq, PartialEq)]
#[struct_deser(identifier = "42", identifier_type = "u8")]
struct Integers {
    u8_0: u8,
    i8_0: i8,
    #[be]
    u16_0: u16,
    #[be]
    i16_0: i16,
    #[le]
    u16_1: u16,
    #[le]
    i16_1: i16,
    #[be]
    u32_0: u32,
    #[be]
    i32_0: i32,
    #[le]
    u32_1: u32,
    #[le]
    i32_1: i32,
    #[be]
    u64_0: u64,
    #[be]
    i64_0: i64,
    #[le]
    u64_1: u64,
    #[le]
    i64_1: i64,
}

#[test]
fn main() {
    use struct_deser::{SerializedByteLen,Identifier,FromBytes,IntoBytes};

    assert_eq!(Integers::IDENTIFIER, 42);
    assert_eq!(Integers::BYTE_LEN, 58);

    let integers = Integers {
        u8_0: 42,
        i8_0: 43,
        u16_0: 44,
        i16_0: 45,
        u16_1: 46,
        i16_1: 47,
        u32_0: 48,
        i32_0: 49,
        u32_1: 50,
        i32_1: 51,
        u64_0: 52,
        i64_0: 53,
        u64_1: 54,
        i64_1: 55,
    };

    let mut bytes = [0; Integers::BYTE_LEN];
    integers.into_bytes(&mut bytes);
    let integers2 = Integers::from_bytes(&bytes);

    assert_eq!(integers, integers2);
}
