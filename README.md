Struct (de)serialization
========================

This crate provides very simple way of (de)serializing structs. The main purpose is to aid working with packets.

Maintenance status
------------------

As you can see from the history of the repository, there wasn't much activity in a while. The main reason for it is that the crate satisfies my needs well and nobody approached me with feature requests, bug repports or pull requests, so I had no way of knowing if people are missing something.

However, in the meantime @xoac created [his own fork of this crate](https://crates.io/crates/endian_codec), which may be more interesting to you. Please go check it first. It's likely that he has more time to maintain his fork than me, so approach him with feature requests first. If he happens to become unresponsive, you may try your luck pinging me. I should have enough time to merge good PRs at least once per week.

Features
--------

* derive(StructDeser)
* handling of endianess
* associated consts
* `no_std`

Usage
-----

```rust
extern crate struct_deser;
#[macro_use]
extern crate struct_deser_derive;

// derive traits
#[derive(StructDeser, Debug, Eq, PartialEq)]
struct Packet {
    // mark as big endian
    // this is mandatory because u16 has multiple bytes
    #[be]
    version: u16,
    // u8 goes without endianess attribute
    ttl: u8,
    // mark as little endian
    #[le]
    chksum: u32,
}

fn main() {
    use struct_deser::{SerializedByteLen,FromBytes,IntoBytes};

    let packet0 = Packet {
        version: 1,
        ttl: 42,
        chksum: 47,
    };

    let mut bytes = [0; Packet::BYTE_LEN];
    packet0.into_bytes(&mut bytes);
    let packet1 = Packet::from_bytes(&bytes);

    assert_eq!(packet0, packet1);
}
```

License
-------
MITNFA
