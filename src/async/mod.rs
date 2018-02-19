#[cfg(feature = "stream")]
mod stream;
mod channel;

use super::data::*;

use std::path::PathBuf;

pub(crate) type SendData = (String, Line, Pos);

pub(crate) fn thread_namer(path: &PathBuf) -> String {
    format!(
        "chase-thread-{}",
        path.to_str().unwrap_or("undisplayable-path")
    )
}
