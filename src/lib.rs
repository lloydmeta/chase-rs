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
//! # Examples
//!
//! This is how you would use Chase (w/ standard lib Channels, though
//! Future Stream usage, available through a feature, is similar)
//!
//! ```
//! # extern crate chase;
//! # extern crate tempdir;
//! # use chase::*;
//! # use tempdir::*;
//! # use std::io::Write;
//! # use std::fs::OpenOptions;
//! # fn main () {
//! let temp_dir = TempDir::new("chase-test").unwrap();
//! let file_path = temp_dir.path().join("test.log");
//! let chaser = Chaser::new(&file_path);
//!
//! let mut file_write = OpenOptions::new()
//! .write(true)
//! .append(true)
//! .create(true)
//! .open(&file_path)
//! .unwrap();
//!
//! write!(file_write, "Hello, world 1\n").unwrap();
//! write!(file_write, "Hello, world 2\n").unwrap();
//!
//! let mut seen = String::new();
//!
//! let (receiver, _) = chaser.run_channel().unwrap();
//!
//! seen.push_str(&receiver.recv().unwrap().0);
//! seen.push_str(&receiver.recv().unwrap().0);
//!
//! assert_eq!(seen.as_str(), "Hello, world 1Hello, world 2");
//!
//! write!(file_write, "Hello, world 3\n").unwrap();
//! seen.push_str(&receiver.recv().unwrap().0);
//! assert_eq!(seen.as_str(), "Hello, world 1Hello, world 2Hello, world 3");
//!
//! drop(receiver);
//! drop(file_write);
//! temp_dir.close().unwrap();
//! # }
//! ```
//!
#[cfg(feature = "stream")]
extern crate futures;

#[cfg(test)]
extern crate tempdir;

mod data;
mod sync;
mod async;
mod errors;

#[cfg(feature = "with-serde")]
#[cfg_attr(feature = "with-serde", macro_use)]
extern crate serde_derive;

pub use data::{Chaser, Line, Pos, DEFAULT_NOT_ROTATED_WAIT_MILLIS,
               DEFAULT_ROTATION_CHECK_WAIT_MILLIS};

pub use errors::ChaseError;
