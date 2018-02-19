#[cfg(feature="stream")]
mod stream;
mod channel;

use super::data::*;

pub(crate) type SendData = (String, Line, Pos);

pub(crate) fn thread_namer(path: &str) -> String {
    format!("chase-thread-{}", path)
}