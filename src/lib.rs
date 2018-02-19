#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Chase: File tailing
//!
//! Chase a file through thick and thin:
//!
//!   * Get back line numbers with every line
//!   * Configurable
//!   * File rotations
//!   * Sync and async modes (incl. support for Future Streams)
//!
#[cfg(feature="stream")]
extern crate futures;

mod data;
mod sync;
mod async;
mod errors;

#[cfg(feature="with-serde")]
#[cfg_attr(feature = "with-serde", macro_use)]
extern crate serde_derive;

pub use data::{Chaser, Line, Pos, DEFAULT_NOT_ROTATED_WAIT_MILLIS,
               DEFAULT_ROTATION_CHECK_WAIT_MILLIS};

pub use errors::ChaseError;
