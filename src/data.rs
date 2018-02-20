//! Holds various data structures used for following files

use std::io::BufReader;
use std::fs::File;
use std::time::Duration;

use std::path::PathBuf;

pub const DEFAULT_ROTATION_CHECK_WAIT_MILLIS: u64 = 100;
pub const DEFAULT_NOT_ROTATED_WAIT_MILLIS: u64 = 50;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Line(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Pos(pub u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub(crate) struct FileId(pub(crate) u64);

/// Your entry point for following a file.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Chaser {
    /// Line to start chasing from
    pub line: Line,
    /// Path of the file you want to chase
    pub path: PathBuf,
    /// When we start running and there is no file and/or file info to be read, how long to
    /// wait before retrying
    pub initial_no_file_wait: Duration,
    /// When we start running and there is no file and/or file info to be read, how many
    /// times to keep trying. None means no limit.
    pub initial_no_file_attempts: Option<usize>,
    /// When we trying to detect a file rotation  and there is no file and/or file info to be
    /// read, how long to wait before retrying
    pub rotation_check_wait: Duration,
    /// When we trying to detect a file rotation  and there is no file and/or file info to be
    /// read, how many times to keep trying. None means no limit.
    pub rotation_check_attempts: Option<usize>,
    /// After we read a file to its end, how long to wait before trying to read the next line
    /// again.
    pub not_rotated_wait: Duration,
}

#[derive(Debug)]
pub(crate) struct Chasing<'a> {
    pub(crate) chaser: &'a mut Chaser,
    pub(crate) file_id: FileId,
    pub(crate) reader: BufReader<File>,
    pub(crate) buffer: String,
    pub(crate) line: Line,
    pub(crate) pos: Pos,
}

impl Chaser {
    /// Creates a new Chaser with default options
    pub fn new<S>(path: S) -> Chaser
    where
        S: Into<PathBuf>,
    {
        Chaser {
            line: Line(0),
            path: path.into(),
            initial_no_file_attempts: None,
            initial_no_file_wait: Duration::from_millis(DEFAULT_ROTATION_CHECK_WAIT_MILLIS),
            rotation_check_attempts: None,
            rotation_check_wait: Duration::from_millis(DEFAULT_ROTATION_CHECK_WAIT_MILLIS),
            not_rotated_wait: Duration::from_millis(DEFAULT_NOT_ROTATED_WAIT_MILLIS),
        }
    }
}
