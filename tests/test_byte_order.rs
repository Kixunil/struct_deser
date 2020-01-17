extern crate struct_deser;
#[macro_use]
extern crate struct_deser_derive;

macro_rules! test_primitive {
    ($primitive:ty, $name:ident, $val:expr, $serialized:expr) => {
        #[derive(StructDeser, Debug, Eq, PartialEq)]
        struct $name {
            #[le]
            le: $primitive,
            #[be]
            be: $primitive,
        }

        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            use struct_deser::{IntoBytes, SerializedByteLen};

            let val = $val;
            let test_val = $name { be: val, le: val };
            let mut bytes = [0; $name::BYTE_LEN];
            test_val.into_bytes(&mut bytes);

            assert_eq!(&bytes, &$serialized);
        }
    };
}

// These tests check whether byte ordering isn't changed due to mistake in implementation
test_primitive!(u16, U16, 42, [42, 0, 0, 42]);
test_primitive!(u32, U32, 42, [42, 0, 0, 0, 0, 0, 0, 42]);
test_primitive!(
    u64,
    U64,
    42,
    [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42]
);
test_primitive!(i16, I16, 42, [42, 0, 0, 42]);
test_primitive!(i32, I32, 42, [42, 0, 0, 0, 0, 0, 0, 42]);
test_primitive!(
    i64,
    I64,
    42,
    [42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42]
);
