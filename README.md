<img src="logo.svg" width="48">

# Identifier

[![doc](https://img.shields.io/badge/docs.rs-identifier-191f26?logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/identifier)
[![minimum rustc 1.31.0](https://img.shields.io/badge/minimum%20rustc-1.31.0-f74c00?logo=rust)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
[![version](https://img.shields.io/crates/v/identifier?color=3b6837&logo=rust)](https://crates.io/crates/identifier)
[![GitHub MathieuTricoire/identifier](https://img.shields.io/badge/GitHub-MathieuTricoire%2Fidentifier-9b88bb?logo=github)](https://github.com/time-rs/time)

_Generate 128 bits id structs easily_

## Installation

```toml
[dependencies]
identifier = "0.1"
```

Version requirement: rustc 1.31+

## Simple Example

_Go to the [generator](#generator) section to see how to create one_

```rust
use generators::uuid;
use identifier::{Display, FromStr, Identifier};

#[derive(Identifier, Eq, PartialEq, Display, FromStr)]
#[identifier(with = "uuid")]
pub struct UserId(u128);

fn main() {
    let id = UserId::generate();
    println!("generated user id: {}", id);

    let parsed_id = "5ed9a942-223e-4639-a97c-6b6c41ac48d3".parse::<UserId>();
    assert!(parsed_id.is_ok());
}
```

## Example with a generator requiring params

```rust
use generators::random_id;
use identifier::{Display, FromStr, Identifier};

const KIND: u32 = 0x1111_ffff;

#[derive(Identifier, Eq, PartialEq, FromStr, Display)]
#[identifier(with = "random_id", params = "KIND, 0xabcd")]
pub struct PostId(u128);

fn main() {
    let id = PostId::generate();
    println!("generated post id: {}", id);
}
```

## Generator

_No params (with the [uuid crate](https://crates.io/crates/uuid))_

```rust
mod uuid {
    use uuid::Uuid;

    pub fn generate() -> u128 {
        Uuid::new_v4().as_u128()
    }

    pub fn validate(value: u128) -> bool {
        if let Some(Version::Random) = Uuid::from_u128(value).get_version() {
            true
        } else {
            false
        }
    }
}
```

_With params_

```rust
mod random_id {
    pub fn generate(kind: u32, seed: u16) -> u128 {
        /* generate a random id with the given params ... */
    }

    pub fn validate(value: u128, kind: u32, _: u16) -> bool {
        /* check it's a correct random id ... */
    }
}
```

---

# License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
