//! Simple serialization and deserialization of structs.
//!
//! The aim of this crate is to help with implementing simple cases of (de)serialization of structs
//! where the whole deserialization only consists of copying fixed amounts of bytes in appropriate
//! byte order. In other words, the cases which are sometimes handled by casting a (packed) struct
//! to array and vice-versa.
//!
//! This crate should be used with `struct_deser-derive`, to avoid writing boilerplate.
//!
//! # Example
//!
//! ```
//! extern crate struct_deser;
//! #[macro_use]
//! extern crate struct_deser_derive;
//!
//! // derive traits
//! #[derive(StructDeser, Debug, Eq, PartialEq)]
//! struct Packet {
//!     // mark as big endian
//!     // this is mandatory because u16 has multiple bytes
//!     #[be]
//!     version: u16,
//!     // u8 goes without endianess attribute
//!     ttl: u8,
//!     // mark as little endian
//!     #[le]
//!     chksum: u32,
//! }
//!
//! fn main() {
//!     use struct_deser::{SerializedByteLen,FromBytes,IntoBytes};
//!
//!     let packet0 = Packet {
//!         version: 1,
//!         ttl: 42,
//!         chksum: 47,
//!     };
//!
//!     let mut bytes = [0; Packet::BYTE_LEN];
//!     packet0.into_bytes(&mut bytes);
//!     let packet1 = Packet::from_bytes(&bytes);
//!
//!     assert_eq!(packet0, packet1);
//! }
//! ```

#![no_std]

extern crate byteorder as byteorder_real;

use byteorder_real::ByteOrder;

/// Re-exported essential items from `byteorder` crate.
/// This is intended mostly for `struct_deser-derive`.
pub mod byteorder {
    pub use byteorder_real::ByteOrder;
    pub use byteorder_real::BE;
    pub use byteorder_real::LE;
}

/// Defines length (number of bytes) of struct when serialized.
///
/// It's used by other traits, so it guarantees that the sizes are same.
pub trait SerializedByteLen {
    /// How many bytes the struct occupies on wire.
    const BYTE_LEN: usize;
}

/// Represents types that can be constructed from bytes.
pub trait FromBytes: SerializedByteLen {
    /// Creates `Self` by deserializing from bytes.
    fn from_bytes(bytes: &[u8]) -> Self;
}

/// Represents types that can be serialized into bytes.
pub trait IntoBytes: SerializedByteLen {
    /// Serializes `self`.
    /// This function must write to the provided slice.
    fn into_bytes(&self, bytes: &mut [u8]);
}

/// Represents types that can be constructed from bytes with specific endianess.
pub trait FromBytesOrdered: SerializedByteLen {
    /// Creates `Self` by deserializing from bytes using byte order.
    fn from_bytes<BO: ByteOrder>(bytes: &[u8]) -> Self;
}

/// Represents types that can be serialized into bytes with specific endianess.
pub trait IntoBytesOrdered: SerializedByteLen {
    /// Serializes `self` using byte order.
    /// This function must write to the provided slice.
    fn into_bytes<BO: ByteOrder>(&self, bytes: &mut [u8]);
}

macro_rules! impl_from_into_bytes {
    ($type:ty, $byte_len:expr, $from:ident, $into:ident) => {
        impl SerializedByteLen for $type {
            const BYTE_LEN: usize = $byte_len;
        }

        impl FromBytesOrdered for $type {
            fn from_bytes<BO: ByteOrder>(bytes: &[u8]) -> Self {
                BO::$from(bytes)
            }
        }

        impl IntoBytesOrdered for $type {
            fn into_bytes<BO: ByteOrder>(&self, bytes: &mut [u8]) {
                BO::$into(bytes, *self)
            }
        }
    };
}

impl SerializedByteLen for u8 {
    const BYTE_LEN: usize = 1;
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0]
    }
}

impl IntoBytes for u8 {
    fn into_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = *self
    }
}

impl SerializedByteLen for i8 {
    const BYTE_LEN: usize = 1;
}

impl FromBytes for i8 {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0] as i8
    }
}

impl IntoBytes for i8 {
    fn into_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = *self as u8
    }
}

impl_from_into_bytes!(u16, 2, read_u16, write_u16);
impl_from_into_bytes!(i16, 2, read_i16, write_i16);
impl_from_into_bytes!(u32, 4, read_u32, write_u32);
impl_from_into_bytes!(i32, 4, read_i32, write_i32);
impl_from_into_bytes!(u64, 8, read_u64, write_u64);
impl_from_into_bytes!(i64, 8, read_i64, write_i64);

/// This trait can be used for marking specific implementation with a constant, which can be used
/// for matching, when determinint the type of message.
/// This doesn't influence derived (de)serialization in any way.
pub trait Identifier {
    /// Type of the identifier.
    type IdentifierType;

    /// The identifier.
    const IDENTIFIER: Self::IdentifierType;
}

macro_rules! impl_byte_arr {
    ($len:expr) => {
        impl SerializedByteLen for [u8; $len] {
            const BYTE_LEN: usize = $len;
        }

        impl FromBytes for [u8; $len] {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0; $len];
                arr.copy_from_slice(&bytes);
                arr
            }
        }

        impl IntoBytes for [u8; $len] {
            fn into_bytes(&self, bytes: &mut [u8]) {
                bytes.copy_from_slice(self)
            }
        }
    };
}

impl_byte_arr!(0);
impl_byte_arr!(1);
impl_byte_arr!(2);
impl_byte_arr!(3);
impl_byte_arr!(4);
impl_byte_arr!(5);
impl_byte_arr!(6);
impl_byte_arr!(7);
impl_byte_arr!(8);
impl_byte_arr!(9);
impl_byte_arr!(10);
impl_byte_arr!(11);
impl_byte_arr!(12);
impl_byte_arr!(13);
impl_byte_arr!(14);
impl_byte_arr!(15);
impl_byte_arr!(16);
impl_byte_arr!(17);
impl_byte_arr!(18);
impl_byte_arr!(19);
impl_byte_arr!(20);
impl_byte_arr!(21);
impl_byte_arr!(22);
impl_byte_arr!(23);
impl_byte_arr!(24);
impl_byte_arr!(25);
impl_byte_arr!(26);
impl_byte_arr!(27);
impl_byte_arr!(28);
impl_byte_arr!(29);
impl_byte_arr!(30);
impl_byte_arr!(31);
impl_byte_arr!(32);
impl_byte_arr!(33);
impl_byte_arr!(34);
impl_byte_arr!(35);
impl_byte_arr!(36);
impl_byte_arr!(37);
impl_byte_arr!(38);
impl_byte_arr!(39);
impl_byte_arr!(40);
impl_byte_arr!(41);
impl_byte_arr!(42);
impl_byte_arr!(43);
impl_byte_arr!(44);
impl_byte_arr!(45);
impl_byte_arr!(46);
impl_byte_arr!(47);
impl_byte_arr!(48);
impl_byte_arr!(49);
impl_byte_arr!(50);
impl_byte_arr!(51);
impl_byte_arr!(52);
impl_byte_arr!(53);
impl_byte_arr!(54);
impl_byte_arr!(55);
impl_byte_arr!(56);
impl_byte_arr!(57);
impl_byte_arr!(58);
impl_byte_arr!(59);
impl_byte_arr!(60);
impl_byte_arr!(61);
impl_byte_arr!(62);
impl_byte_arr!(63);
impl_byte_arr!(64);
impl_byte_arr!(65);
impl_byte_arr!(66);
impl_byte_arr!(67);
impl_byte_arr!(68);
impl_byte_arr!(69);
impl_byte_arr!(70);
impl_byte_arr!(71);
impl_byte_arr!(72);
impl_byte_arr!(73);
impl_byte_arr!(74);
impl_byte_arr!(75);
impl_byte_arr!(76);
impl_byte_arr!(77);
impl_byte_arr!(78);
impl_byte_arr!(79);
impl_byte_arr!(80);
impl_byte_arr!(81);
impl_byte_arr!(82);
impl_byte_arr!(83);
impl_byte_arr!(84);
impl_byte_arr!(85);
impl_byte_arr!(86);
impl_byte_arr!(87);
impl_byte_arr!(88);
impl_byte_arr!(89);
impl_byte_arr!(90);
impl_byte_arr!(91);
impl_byte_arr!(92);
impl_byte_arr!(93);
impl_byte_arr!(94);
impl_byte_arr!(95);
impl_byte_arr!(96);
impl_byte_arr!(97);
impl_byte_arr!(98);
impl_byte_arr!(99);
impl_byte_arr!(100);
impl_byte_arr!(101);
impl_byte_arr!(102);
impl_byte_arr!(103);
impl_byte_arr!(104);
impl_byte_arr!(105);
impl_byte_arr!(106);
impl_byte_arr!(107);
impl_byte_arr!(108);
impl_byte_arr!(109);
impl_byte_arr!(110);
impl_byte_arr!(111);
impl_byte_arr!(112);
impl_byte_arr!(113);
impl_byte_arr!(114);
impl_byte_arr!(115);
impl_byte_arr!(116);
impl_byte_arr!(117);
impl_byte_arr!(118);
impl_byte_arr!(119);
impl_byte_arr!(120);
impl_byte_arr!(121);
impl_byte_arr!(122);
impl_byte_arr!(123);
impl_byte_arr!(124);
impl_byte_arr!(125);
impl_byte_arr!(126);
impl_byte_arr!(127);
impl_byte_arr!(128);
impl_byte_arr!(129);
impl_byte_arr!(130);
impl_byte_arr!(131);
impl_byte_arr!(132);
impl_byte_arr!(133);
impl_byte_arr!(134);
impl_byte_arr!(135);
impl_byte_arr!(136);
impl_byte_arr!(137);
impl_byte_arr!(138);
impl_byte_arr!(139);
impl_byte_arr!(140);
impl_byte_arr!(141);
impl_byte_arr!(142);
impl_byte_arr!(143);
impl_byte_arr!(144);
impl_byte_arr!(145);
impl_byte_arr!(146);
impl_byte_arr!(147);
impl_byte_arr!(148);
impl_byte_arr!(149);
impl_byte_arr!(150);
impl_byte_arr!(151);
impl_byte_arr!(152);
impl_byte_arr!(153);
impl_byte_arr!(154);
impl_byte_arr!(155);
impl_byte_arr!(156);
impl_byte_arr!(157);
impl_byte_arr!(158);
impl_byte_arr!(159);
impl_byte_arr!(160);
impl_byte_arr!(161);
impl_byte_arr!(162);
impl_byte_arr!(163);
impl_byte_arr!(164);
impl_byte_arr!(165);
impl_byte_arr!(166);
impl_byte_arr!(167);
impl_byte_arr!(168);
impl_byte_arr!(169);
impl_byte_arr!(170);
impl_byte_arr!(171);
impl_byte_arr!(172);
impl_byte_arr!(173);
impl_byte_arr!(174);
impl_byte_arr!(175);
impl_byte_arr!(176);
impl_byte_arr!(177);
impl_byte_arr!(178);
impl_byte_arr!(179);
impl_byte_arr!(180);
impl_byte_arr!(181);
impl_byte_arr!(182);
impl_byte_arr!(183);
impl_byte_arr!(184);
impl_byte_arr!(185);
impl_byte_arr!(186);
impl_byte_arr!(187);
impl_byte_arr!(188);
impl_byte_arr!(189);
impl_byte_arr!(190);
impl_byte_arr!(191);
impl_byte_arr!(192);
impl_byte_arr!(193);
impl_byte_arr!(194);
impl_byte_arr!(195);
impl_byte_arr!(196);
impl_byte_arr!(197);
impl_byte_arr!(198);
impl_byte_arr!(199);
impl_byte_arr!(200);
impl_byte_arr!(201);
impl_byte_arr!(202);
impl_byte_arr!(203);
impl_byte_arr!(204);
impl_byte_arr!(205);
impl_byte_arr!(206);
impl_byte_arr!(207);
impl_byte_arr!(208);
impl_byte_arr!(209);
impl_byte_arr!(210);
impl_byte_arr!(211);
impl_byte_arr!(212);
impl_byte_arr!(213);
impl_byte_arr!(214);
impl_byte_arr!(215);
impl_byte_arr!(216);
impl_byte_arr!(217);
impl_byte_arr!(218);
impl_byte_arr!(219);
impl_byte_arr!(220);
impl_byte_arr!(221);
impl_byte_arr!(222);
impl_byte_arr!(223);
impl_byte_arr!(224);
impl_byte_arr!(225);
impl_byte_arr!(226);
impl_byte_arr!(227);
impl_byte_arr!(228);
impl_byte_arr!(229);
impl_byte_arr!(230);
impl_byte_arr!(231);
impl_byte_arr!(232);
impl_byte_arr!(233);
impl_byte_arr!(234);
impl_byte_arr!(235);
impl_byte_arr!(236);
impl_byte_arr!(237);
impl_byte_arr!(238);
impl_byte_arr!(239);
impl_byte_arr!(240);
impl_byte_arr!(241);
impl_byte_arr!(242);
impl_byte_arr!(243);
impl_byte_arr!(244);
impl_byte_arr!(245);
impl_byte_arr!(246);
impl_byte_arr!(247);
impl_byte_arr!(248);
impl_byte_arr!(249);
impl_byte_arr!(250);
impl_byte_arr!(251);
impl_byte_arr!(252);
impl_byte_arr!(253);
impl_byte_arr!(254);
impl_byte_arr!(255);
impl_byte_arr!(256);
impl_byte_arr!(512);
impl_byte_arr!(1024);
impl_byte_arr!(2048);
impl_byte_arr!(4096);
impl_byte_arr!(8192);
impl_byte_arr!(16384);
impl_byte_arr!(32768);
impl_byte_arr!(65536);
impl_byte_arr!(131072);
impl_byte_arr!(262144);
impl_byte_arr!(524288);
impl_byte_arr!(1048576);
impl_byte_arr!(2097152);
impl_byte_arr!(4194304);
