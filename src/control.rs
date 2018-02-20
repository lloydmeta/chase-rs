//! Holds control constructs for continuing or exiting the synchronous
//! watch loop

/// When chasing a file synchronously, use this to control when to exit the
/// follow loop.
#[derive(PartialEq, Eq, Debug)]
pub enum Control {
    Stop,
    Continue,
}
