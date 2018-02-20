//! Holds a synchronous implementation of file following.

use data::*;
use control::*;
use errors::ChaseError;

use std::io::{self, BufReader, SeekFrom};
use std::io::prelude::*;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

impl Chaser {
    /// Start chasing a file synchronously.
    ///
    /// The provided callback function will be invoked whenever a line
    /// is read.
    ///
    /// ```
    /// # extern crate chase;
    /// # extern crate tempdir;
    /// # use chase::*;
    /// # use tempdir::*;
    /// # use std::io::Write;
    /// # use std::fs::OpenOptions;
    /// # fn main () {
    /// let temp_dir = TempDir::new("chase-test-sync-docs").unwrap();
    /// let file_path = temp_dir.path().join("test.log");
    /// let mut chaser = Chaser::new(&file_path);
    ///
    /// let mut file_write = OpenOptions::new()
    ///   .write(true)
    ///   .append(true)
    ///   .create(true)
    ///   .open(&file_path)
    ///   .unwrap();
    ///
    /// write!(file_write, "Hello, world 1\n").unwrap();
    /// write!(file_write, "Hello, world 2\n").unwrap();
    /// write!(file_write, "Hello, world 3\n").unwrap();
    ///
    /// let mut seen = Vec::with_capacity(3);
    ///
    /// // This is a synchronous loop; so we need to exit manually
    /// chaser.run(|line, _, _| {
    ///     seen.push(line.to_string());
    ///     if seen.len() < 3 {
    ///         Ok(Control::Continue)
    ///     } else {
    ///         Ok(Control::Stop)
    ///     }
    /// }).unwrap();
    ///
    /// assert_eq!(seen, vec!["Hello, world 1".to_string(), "Hello, world 2".to_string(), "Hello, world 3".to_string()]);
    /// drop(file_write);
    /// temp_dir.close().unwrap();
    /// # }
    /// ```
    pub fn run<F>(&mut self, mut f: F) -> Result<(), ChaseError>
    where
        F: FnMut(&str, Line, Pos) -> Result<Control, ChaseError>,
    {
        let file = {
            let attempts = self.initial_no_file_attempts;
            let wait = self.initial_no_file_wait;
            try_until(|| File::open(&self.path), attempts, Some(wait))?
        };
        let file_id = get_file_id(&file)?;
        // Create a BufReader and skip to the proper line number while
        // keeping track of byte-position
        let mut reader = BufReader::new(file);
        let mut current_line = Line(0);
        let mut current_pos = Pos(0);
        let mut buffer = String::new();
        'skip_to_line: while current_line < self.line {
            let read_bytes = reader.read_line(&mut buffer)? as u64;
            if read_bytes > 0 {
                current_pos.0 += read_bytes;
                current_line.0 += 1;
                buffer.clear();
                reader.seek(SeekFrom::Start(current_pos.0))?;
            } else {
                break 'skip_to_line;
            }
        }

        let mut running = Chasing {
            chaser: self,
            file_id,
            reader,
            buffer,
            pos: current_pos,
            line: current_line,
        };
        chase(&mut running, &mut f, false)
    }
}

fn chase<F>(running: &mut Chasing, f: &mut F, grabbing_remainder: bool) -> Result<(), ChaseError>
where
    F: FnMut(&str, Line, Pos) -> Result<Control, ChaseError>,
{
    'reading: loop {
        'read_to_eof: loop {
            let bytes_read = running.reader.read_line(&mut running.buffer)?;
            if bytes_read > 0 {
                let control = f(
                    running.buffer.trim_right_matches('\n'),
                    running.line,
                    running.pos,
                )?;
                if control == Control::Stop {
                    break 'reading;
                }
                running.buffer.clear();
                running.line.0 += 1;
                running.pos.0 += bytes_read as u64;
                running.reader.seek(SeekFrom::Start(running.pos.0))?;
            } else {
                break 'read_to_eof; // no bytes read -> EOF
            }
        }

        if grabbing_remainder {
            break 'reading;
        } else {
            let rotation_status = {
                let attempts = running.chaser.rotation_check_attempts;
                let wait = running.chaser.rotation_check_wait;
                try_until(|| check_rotation_status(running), attempts, Some(wait))?
            };
            match rotation_status {
                RotationStatus::Rotated {
                    file: new_file,
                    file_id: new_file_id,
                } => {
                    // Read the rest of the same file
                    chase(running, f, true)?;
                    // Restart reading loop, but read from the top
                    running.line = Line(0);
                    running.pos = Pos(0);
                    running.file_id = new_file_id;
                    running.reader = BufReader::new(new_file);
                    continue 'reading;
                }
                RotationStatus::NotRotated => {
                    sleep(running.chaser.not_rotated_wait);
                    continue 'reading;
                }
            }
        }
    }
    Ok(())
}

fn check_rotation_status(running: &mut Chasing) -> Result<RotationStatus, io::Error> {
    let file = File::open(&running.chaser.path)?;
    let file_id = get_file_id(&file)?;
    if file_id != running.file_id {
        Ok(RotationStatus::Rotated { file, file_id })
    } else {
        Ok(RotationStatus::NotRotated)
    }
}

// Will go at least once, max attempts set to None means try until successful
fn try_until<R, E, F>(
    mut f: F,
    max_attempts: Option<usize>,
    delay: Option<Duration>,
) -> Result<R, E>
where
    F: FnMut() -> Result<R, E>,
{
    let mut tries = 0;
    loop {
        let current_try = f();
        if max_attempts.is_some() {
            tries += 1;
        }
        if current_try.is_err() && max_attempts.map(|until| tries < until).unwrap_or(true) {
            if let Some(duration) = delay {
                sleep(duration);
            }
            continue;
        } else {
            return current_try;
        }
    }
}

#[cfg(unix)]
fn get_file_id(file: &File) -> Result<FileId, io::Error> {
    let meta = file.metadata()?;
    Ok(FileId(meta.ino()))
}

enum RotationStatus {
    Rotated { file: File, file_id: FileId },
    NotRotated,
}

#[cfg(test)]
mod tests {

    use sync::try_until;
    use data::*;
    use control::*;
    use tempdir::*;
    use std::io::Write;

    use std::fs::OpenOptions;

    #[test]
    fn try_until_test() {
        let result_0: Result<i32, ()> = try_until(|| Ok(1), None, None);
        assert_eq!(result_0, Ok(1));
        let result_1: Result<i32, ()> = try_until(|| Ok(1), Some(1), None);
        assert_eq!(result_1, Ok(1));
        let mut tries = 0;
        let result_2: Result<i32, ()> = try_until(
            || {
                tries += 1;
                Err(())
            },
            Some(1),
            None,
        );
        assert_eq!(tries, 1);
        assert_eq!(result_2, Err(()));
        let result_3: Result<i32, ()> = try_until(
            || {
                tries += 1;
                if tries < 1000 {
                    Err(())
                } else {
                    Ok(1)
                }
            },
            Some(999),
            None,
        );
        assert_eq!(result_3, Ok(1));
    }

    #[test]
    fn run_channel_test() {
        let temp_dir = TempDir::new("chase-test-sync").unwrap();
        let file_path = temp_dir.path().join("test.log");
        let mut chaser = Chaser::new(&file_path);

        let mut file_write = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&file_path)
            .unwrap();

        write!(file_write, "Hello, world 1\n").unwrap();
        write!(file_write, "Hello, world 2\n").unwrap();
        write!(file_write, "Hello, world 3\n").unwrap();

        let mut seen = Vec::with_capacity(3);

        // This is a synchronous loop; so we need to exit manually
        chaser
            .run(|line, _, _| {
                seen.push(line.to_string());
                if seen.len() < 3 {
                    Ok(Control::Continue)
                } else {
                    Ok(Control::Stop)
                }
            })
            .unwrap();

        assert_eq!(
            seen,
            vec![
                "Hello, world 1".to_string(),
                "Hello, world 2".to_string(),
                "Hello, world 3".to_string(),
            ]
        );
        drop(file_write);
        temp_dir.close().unwrap();
    }
}
