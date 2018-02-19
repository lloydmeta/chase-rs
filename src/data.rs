use std::io::BufReader;
use std::fs::File;
use std::time::Duration;

pub const DEFAULT_ROTATION_CHECK_WAIT_MILLIS: u64 = 100;
pub const DEFAULT_NOT_ROTATED_WAIT_MILLIS: u64 = 100;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Line(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Pos(pub u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub(crate) struct FileId(pub(crate) u64);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Chaser {
    pub(crate) line: Line,
    pub(crate) path: String,
    pub(crate) rotation_check_wait: Duration,
    pub(crate) rotation_check_attempts: Option<usize>,
    pub(crate) not_rotated_wait: Duration,
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
    pub fn new(path: &str) -> Chaser {
        Chaser {
            line: Line(0),
            path: path.to_string(),
            rotation_check_attempts: None,
            rotation_check_wait: Duration::from_millis(DEFAULT_ROTATION_CHECK_WAIT_MILLIS),
            not_rotated_wait: Duration::from_millis(DEFAULT_NOT_ROTATED_WAIT_MILLIS),
        }
    }

    pub fn set_path(&mut self, path: &str) -> () {
        self.path = path.to_string();
    }

    pub fn get_path(&self) -> &str {
        self.path.as_ref()
    }

    pub fn get_line(&self) -> Line {
        self.line
    }

    pub fn set_line(&mut self, line: Line) -> () {
        self.line = line;
    }

    pub fn get_rotation_check_wait(&self) -> &Duration {
        &self.rotation_check_wait
    }

    pub fn set_rotation_check_wait(&mut self, duration: Duration) -> () {
        self.rotation_check_wait = duration;
    }
    pub fn get_not_rotated_wait(&self) -> &Duration {
        &self.not_rotated_wait
    }

    pub fn set_not_rotated_wait(&mut self, duration: Duration) -> () {
        self.not_rotated_wait = duration;
    }

    pub fn get_rotation_check_attempts(&self) -> Option<usize> {
        self.rotation_check_attempts
    }

    pub fn set_rotation_check_attempts(&mut self, attempts: Option<usize>) -> () {
        self.rotation_check_attempts = attempts;
    }
}