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
    pub use ::byteorder_real::LE;
    pub use ::byteorder_real::BE;
    pub use byteorder_real::ByteOrder;
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
    }
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
