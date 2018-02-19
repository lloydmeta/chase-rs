//! Holds logic for tailing a file asynchronously using standard
//! channels from the standard lib.

use super::super::data::*;

use super::{thread_namer, SendData};

use std::sync::mpsc::*;
use std::thread::{Builder, JoinHandle};

use errors::ChaseError;

impl Chaser {
    /// Consumes the given chaser and gives you back a standard lib Channel to read
    /// from
    ///
    /// ```
    /// # extern crate chase;
    /// # extern crate tempdir;
    /// # use chase::*;
    /// # use tempdir::*;
    /// # use std::io::Write;
    /// # use std::fs::OpenOptions;
    /// # fn main () {
    /// let temp_dir = TempDir::new("chase-test-channel-docs").unwrap();
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
    /// let mut seen = String::new();
    ///
    /// let (receiver, _) = chaser.run_channel().unwrap();
    ///
    /// seen.push_str(&receiver.recv().unwrap().0);
    /// seen.push_str(&receiver.recv().unwrap().0);
    ///
    /// assert_eq!(seen.as_str(), "Hello, world 1Hello, world 2");
    ///
    /// write!(file_write, "Hello, world 3\n").unwrap();
    /// seen.push_str(&receiver.recv().unwrap().0);
    /// assert_eq!(seen.as_str(), "Hello, world 1Hello, world 2Hello, world 3");
    ///
    /// drop(receiver);
    /// drop(file_write);
    /// temp_dir.close().unwrap();
    /// # }
    /// ```
    pub fn run_channel(
        self,
    ) -> Result<(Receiver<SendData>, JoinHandle<Result<(), ChaseError>>), ChaseError> {
        let (tx, rx) = sync_channel(0);
        let join_handle = Builder::new()
            .name(thread_namer(&self.path))
            .spawn(move || {
                let mut moved_chaser = self;
                moved_chaser.run(|line, num, pos| Ok(tx.send((line.to_string(), num, pos))?))?;
                Ok(())
            })?;
        Ok((rx, join_handle))
    }
}
//
//#[cfg(test)]
//mod tests {
//    use super::super::super::data::*;
//    use tempdir::*;
//    use std::io::Write;
//
//    use std::fs::{rename, OpenOptions};
//
//    #[test]
//    fn run_channel_test() {
//        let temp_dir = TempDir::new("chase-test-channel").unwrap();
//        let file_path = temp_dir.path().join("test.log");
//        let chaser = Chaser::new(&file_path);
//
//        let mut file_write = OpenOptions::new()
//            .write(true)
//            .append(true)
//            .create(true)
//            .open(&file_path)
//            .unwrap();
//
//        write!(file_write, "Hello, world 1\n").unwrap();
//        write!(file_write, "Hello, world 2\n").unwrap();
//
//        let mut seen = String::new();
//
//        let (receiver, _) = chaser.run_channel().unwrap();
//
//        seen.push_str(&receiver.recv().unwrap().0);
//        seen.push_str(&receiver.recv().unwrap().0);
//
//        assert_eq!(seen.as_str(), "Hello, world 1Hello, world 2");
//
//        write!(file_write, "Hello, world 3\n").unwrap();
//        seen.push_str(&receiver.recv().unwrap().0);
//        assert_eq!(seen.as_str(), "Hello, world 1Hello, world 2Hello, world 3");
//
//        // rotation
//        let mut file_write_new = {
//            rename(&file_path, temp_dir.path().join("test.log.bk")).unwrap();
//            OpenOptions::new()
//                .write(true)
//                .append(true)
//                .create(true)
//                .open(&file_path)
//                .unwrap()
//        };
//        write!(file_write_new, "Hello, world 4\n").unwrap();
//
//        seen.push_str(&receiver.recv().unwrap().0);
//        assert_eq!(
//            seen.as_str(),
//            "Hello, world 1Hello, world 2Hello, world 3Hello, world 4"
//        );
//
//        drop(receiver);
//        drop(file_write);
//        temp_dir.close().unwrap();
//    }
//}
