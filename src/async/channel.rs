//! Holds logic for tailing a file asynchronously using standard
//! channels from the standard lib.

use super::super::data::*;

use super::{SendData, thread_namer};

use std::sync::mpsc::*;
use std::thread::{Builder, JoinHandle};

use errors::ChaseError;

impl Chaser {

    /// Consumes the given chaser and gives you back a standard lib Channel to read
    /// from
    pub fn run_channel(
        self,
    ) -> Result<(Receiver<SendData>, JoinHandle<Result<(), ChaseError>>), ChaseError> {
        let (tx, rx) = sync_channel(0);
        let join_handle = Builder::new()
            .name(thread_namer(self.path.as_str()))
            .spawn(move || {
                let mut moved_chaser = self;
                moved_chaser.run(|line, num, pos| Ok(tx.send((line.to_string(), num, pos))?))?;
                Ok(())
            })?;
        Ok((rx, join_handle))
    }
}
