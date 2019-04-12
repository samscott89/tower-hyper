//! A tower hyper bridge library that provides utilties
//! to use hyper with a tower middleware stack.
//!
//! # Overview
//!
//! This library is comprised of client and server modules. Currently, only
//! the client portion is done and working. The server side is blocked partially
//! by hypers use of its own Service and MakeService traits.

#![deny(missing_docs, missing_debug_implementations)]

pub mod body;
pub mod client;
/// Contains ref retry logic
#[cfg(feature = "retry")]
pub mod retry;
pub mod server;
/// Util for working with hyper and tower
pub mod util;


// Known bug in rustc: https://github.com/rust-lang/rust/issues/18290
#[allow(dead_code)]
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
