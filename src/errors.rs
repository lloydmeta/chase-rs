//! Module holding various error wrappers

use std::io;

use std::sync::mpsc as channel_mpsc;

#[cfg(feature = "stream")]
use futures::sync::mpsc as stream_mpsc;

use std::fmt;
use std::error::Error;
use async::SendData;

#[derive(Debug)]
pub enum ChaseError {
    IoError(io::Error),
    ChannelSendError(channel_mpsc::SendError<SendData>),
    #[cfg(feature = "stream")] StreamSendError(stream_mpsc::SendError<SendData>),
}

impl fmt::Display for ChaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ChaseError::*;
        match self {
            &IoError(ref e) => write!(f, "{}", e),
            &ChannelSendError(ref e) => write!(f, "{}", e),
            #[cfg(feature = "stream")]
            &StreamSendError(ref e) => write!(f, "{}", e),
        }
    }
}

impl Error for ChaseError {
    fn description(&self) -> &str {
        use self::ChaseError::*;
        match self {
            &IoError(ref e) => e.description(),
            &ChannelSendError(ref e) => e.description(),
            #[cfg(feature = "stream")]
            &StreamSendError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        use self::ChaseError::*;
        match self {
            &IoError(ref e) => Some(e),
            &ChannelSendError(ref e) => Some(e),
            #[cfg(feature = "stream")]
            &StreamSendError(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for ChaseError {
    fn from(e: io::Error) -> Self {
        ChaseError::IoError(e)
    }
}

impl From<channel_mpsc::SendError<SendData>> for ChaseError {
    fn from(e: channel_mpsc::SendError<SendData>) -> Self {
        ChaseError::ChannelSendError(e)
    }
}

#[cfg(feature = "stream")]
impl From<stream_mpsc::SendError<SendData>> for ChaseError {
    fn from(e: stream_mpsc::SendError<SendData>) -> Self {
        ChaseError::StreamSendError(e)
    }
}
