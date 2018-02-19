//! Holds an implementation of file tailing as a Futures Stream, with back-pressure
//! taken into consideration.

use data::*;

use super::{thread_namer, SendData};

use std::thread::{Builder, JoinHandle};
use futures::{Future, Sink};
use futures::sync::mpsc::*;

use errors::ChaseError;

impl Chaser {

    /// Consume the given Chaser and returns a Stream from which you can
    /// read attempts to read lines from the file
    ///
    /// ```
    /// # extern crate chase;
    /// # extern crate tempdir;
    /// # extern crate futures;
    /// # use chase::*;
    /// # use tempdir::*;
    /// # use std::io::Write;
    /// # use std::fs::OpenOptions;
    /// # use futures::{Future, Stream};
    /// # use futures::future;
    /// # fn main () {
    /// let temp_dir = TempDir::new("chase-test").unwrap();
    /// let file_path = temp_dir.path().join("test.log");
    /// let chaser = Chaser::new(&file_path);
    ///
    /// let mut file_write = OpenOptions::new()
    /// .write(true)
    /// .append(true)
    /// .create(true)
    /// .open(&file_path)
    /// .unwrap();
    ///
    /// write!(file_write, "Hello, world 1\n").unwrap();
    /// write!(file_write, "Hello, world 2\n").unwrap();
    ///
    /// let (stream, _) = chaser.run_stream().unwrap();
    ///
    /// let accumulated = stream
    /// .take(3) // we'll add another one after this is declared to show things are really async
    /// .fold(String::new(), |mut acc, (line, _, _)| {
    /// acc.push_str(&line);
    /// future::ok(acc)
    /// });
    ///
    /// write!(file_write, "Hello, world 3\n").unwrap();
    /// assert_eq!(
    ///     accumulated.wait(),
    ///     Ok("Hello, world 1Hello, world 2Hello, world 3".to_string())
    /// );
    ///
    /// drop(file_write);
    /// temp_dir.close().unwrap();
    /// # }
    /// ```
    pub fn run_stream(
        mut self,
    ) -> Result<(Receiver<SendData>, JoinHandle<Result<(), ChaseError>>), ChaseError> {
        let (mut tx, rx) = channel(0);

        let join_handle = Builder::new()
            .name(thread_namer(&self.path))
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

#[cfg(test)]
mod tests {
    use super::super::super::data::*;
    use tempdir::*;
    use std::io::Write;
    use futures::{Future, Stream};
    use futures::future;

    use std::fs::OpenOptions;

    #[test]
    fn run_stream_test() {
        let temp_dir = TempDir::new("chase-test").unwrap();
        let file_path = temp_dir.path().join("test.log");
        let chaser = Chaser::new(&file_path);

        let mut file_write = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&file_path)
            .unwrap();

        write!(file_write, "Hello, world 1\n").unwrap();
        write!(file_write, "Hello, world 2\n").unwrap();

        let (stream, _) = chaser.run_stream().unwrap();

        let accumulated = stream
            .take(3) // we'll add another one after this is declared to show things are really async
            .fold(String::new(), |mut acc, (line, _, _)| {
                acc.push_str(&line);
                future::ok(acc)
            });

        write!(file_write, "Hello, world 3\n").unwrap();
        assert_eq!(
            accumulated.wait(),
            Ok("Hello, world 1Hello, world 2Hello, world 3".to_string())
        );

        drop(file_write);
        temp_dir.close().unwrap();
    }
}
