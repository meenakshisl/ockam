//! Software implementation of ockam_core::capability traits.
//!
//! This crate contains one of the possible implementation of the capability traits
//! which you can use with Ockam library.
#![deny(unsafe_code)]
#![warn(
    //missing_docs, TODO
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![allow(unused_variables)] // TODO
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

mod capability;
mod error;

pub use capability::*;
pub use error::*;

// https://github.com/twittner/ockam/blob/abac/implementations/rust/ockam/ockam_abac/src/
pub mod abac;
