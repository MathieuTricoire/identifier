//! # Simple Example
//!
//! ```
//! # use identifier::{FromStr, Identifier};
//! #
//! mod generator {
//!     pub fn generate() -> u128 {
//!         0x1234567890_abcdef_1234567890_abcdef
//!     }
//!
//!     pub fn validate(_: u128) -> bool {
//!         true
//!     }
//! }
//!
//! #[derive(Identifier, Eq, PartialEq, FromStr, Debug)]
//! #[identifier(with = "generator")]
//! pub struct UserId(u128);
//!
//! fn main() {
//!     let id = UserId::generate();
//!     let expected_id: UserId = "1234567890-abcdef-1234567890-abcdef".parse().unwrap();
//!     assert_eq!(id, expected_id);
//! }
//! ```
//!
//! ## Example with a generate function requiring params
//!
//! ```
//! # use identifier::{FromStr, Identifier};
//! #
//! mod generator {
//!     pub fn generate(kind: u32, seed: u16) -> u128 {
//!         let random_number = 0x1234_0000 + (seed as u128);
//!         ((kind as u128) << 96) + random_number
//!     }
//!
//!     pub fn validate(value: u128, kind: u32, _: u16) -> bool {
//!         (value >> 96) as u32 == kind
//!     }
//! }
//!
//! const KIND_USER: u32 = 0x1111_ffff;
//!
//! #[derive(Identifier, Eq, PartialEq, FromStr, Debug)]
//! #[identifier(with = "generator", params = "KIND_USER, 0xabcd")]
//! pub struct UserId(u128);
//!
//! fn main() {
//!     let id = UserId::generate();
//!     let expected_id: UserId = "1111ffff-00000000-00000000-1234abcd".parse().unwrap();
//!     assert_eq!(id, expected_id);
//!
//!     let parse_error = "0000ffff-00000000-00000000-1234abcd".parse::<UserId>();
//!     assert!(if let Err(identifier::ParseError::Invalid) = parse_error { true } else { false });
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/MathieuTricoire/identifier/main/logo.svg"
)]

mod error;
mod identifier;

// #[doc(inline)]
pub use crate::error::ParseError;
pub use crate::identifier::Identifier;
pub trait DebugId {}

// Derive
#[allow(unused_imports)]
#[macro_use]
extern crate identifier_derive;
#[doc(hidden)]
pub use identifier_derive::*;
