#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Chase: File following
//!
//! Chase a file through thick and thin:
//!
//!   * Provide line numbers with each line yielded
//!   * Ability to exit the watch loop programmatically
//!   * Deals with file rotations automatically
//!   * Cross-platform async
//!   * Configurable (which line to start on, delays and retries)
//!   * Easy to use synchronously
//!   * Async modes (incl. support for Future Streams)
//!
//! # Examples
//!
//! ## Sync
//!
//! Here, we enter a synchronous watch loop and use Control::Stop and Control::Continue
//! to decide when we want to exit the loop
//!
//! ```
//! # extern crate chase;
//! # extern crate tempdir;
//! # use chase::*;
//! # use tempdir::*;
//! # use std::io::Write;
//! # use std::fs::OpenOptions;
//! # fn main () {
//! let temp_dir = TempDir::new("chase-test-sync-docs").unwrap();
//! let file_path = temp_dir.path().join("test.log");
//! let mut chaser = Chaser::new(&file_path);
//!
//! let mut file_write = OpenOptions::new()
//!   .write(true)
//!   .append(true)
//!   .create(true)
//!   .open(&file_path)
//!   .unwrap();
//!
//! write!(file_write, "Hello, world 1\n").unwrap();
//! write!(file_write, "Hello, world 2\n").unwrap();
//! write!(file_write, "Hello, world 3\n").unwrap();
//!
//! let mut seen = Vec::with_capacity(3);
//!
//! // This is a synchronous loop; so we need to exit manually
//! chaser.run(|line, _, _| {
//!     seen.push(line.to_string());
//!     if seen.len() < 3 {
//!         Ok(Control::Continue)
//!     } else {
//!         Ok(Control::Stop)
//!     }
//! }).unwrap();
//!
//! assert_eq!(seen, vec!["Hello, world 1".to_string(), "Hello, world 2".to_string(), "Hello, world 3".to_string()]);
//! drop(file_write);
//! temp_dir.close().unwrap();
//! # }
//! ```
//!
//! ## Async
//!
//! This is how you would use Chase with standard lib Channels, though
//! Future Stream usage, available through a feature, is similar.
//!
//! In async modes, simply drop the receiving end and the watch loop will stop. Any
//! elements that it tries to send you afterwards will bubble up to you as the Err
//! result of joining the thread, which is the same as how channels normally act.
//!
//! ```
//! # extern crate chase;
//! # extern crate tempdir;
//! # use chase::*;
//! # use tempdir::*;
//! # use std::io::Write;
//! # use std::fs::OpenOptions;
//! # fn main () {
//! let temp_dir = TempDir::new("chase-test-lib").unwrap();
//! let file_path = temp_dir.path().join("test.log");
//! let chaser = Chaser::new(&file_path);
//!
//! let mut file_write = OpenOptions::new()
//!   .write(true)
//!   .append(true)
//!   .create(true)
//!   .open(&file_path)
//!   .unwrap();
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
mod control;

#[cfg(feature = "with-serde")]
#[cfg_attr(feature = "with-serde", macro_use)]
extern crate serde_derive;

pub use data::{Chaser, Line, Pos, DEFAULT_NOT_ROTATED_WAIT_MILLIS,
               DEFAULT_ROTATION_CHECK_WAIT_MILLIS};

pub use errors::ChaseError;

pub use control::Control;
