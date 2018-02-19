//! Holds an implementation of file tailing as a Futures Stream, with back-pressure
//! taken into consideration.

use data::*;

use super::{SendData, thread_namer};

use std::thread::{Builder, JoinHandle};
use futures::{Future, Sink};
use futures::sync::mpsc::*;

use errors::ChaseError;

impl Chaser {
    /// Consume the given Chaser and returns a Stream from which you can
    /// read attempts to read lines from the file
    pub fn run_stream(
        mut self,
    ) -> Result<(Receiver<SendData>, JoinHandle<Result<(), ChaseError>>),
        ChaseError> {
        let (mut tx, rx) = channel(0);

        let join_handle = Builder::new()
            .name(thread_namer(self.path.as_str()))
            .spawn(move || {
                self.run(|line, num, pos| {
                    let next_tx = tx.clone().send((line.to_string(), num, pos)).wait()?;
                    tx = next_tx;
                    Ok(())
                })?;
                Ok(())
            })?;
        Ok((rx, join_handle))
    }
}
