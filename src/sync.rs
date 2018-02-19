//! Holds a synchronous implementation of file tailing.

use data::*;
use errors::ChaseError;

use std::io::{self, BufReader, SeekFrom};
use std::io::prelude::*;
use std::fs::File;
use std::thread::sleep;

#[cfg(any(target_os = "redox", unix))]
use std::os::unix::fs::MetadataExt;

#[cfg(any(target_os = "linux", target_os = "l4re"))]
use std::os::linux::fs::MetadataExt;

impl Chaser {

    /// Start chasing a file synchronously.
    ///
    /// The provided callback function will be invoked whenever a line
    /// is read.
    pub fn run<F>(&mut self, mut f: F) -> Result<(), ChaseError>
    where
        F: FnMut(&str, Line, Pos) -> Result<(), ChaseError>,
    {
        let file = File::open(&self.path)?;
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
    F: FnMut(&str, Line, Pos) -> Result<(), ChaseError>,
{
    'reading: loop {
        'read_to_eof: loop {
            let bytes_read = running.reader.read_line(&mut running.buffer)?;
            if bytes_read > 0 {
                f(
                    running.buffer.trim_right_matches('\n'),
                    running.line,
                    running.pos,
                )?;
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
            let mut rotation_check_attempts: usize = 1;
            'rotation_check: loop {
                match check_rotation_status(running) {
                    Ok(RotationStatus::Rotated {
                        file: new_file,
                        file_id: new_file_id,
                    }) => {
                        // Read the rest of the same file
                        chase(running, f, true)?;
                        // Restart reading loop, but read from the top
                        running.line = Line(0);
                        running.pos = Pos(0);
                        running.file_id = new_file_id;
                        running.reader = BufReader::new(new_file);
                        continue 'reading;
                    }
                    Ok(RotationStatus::NotRotated) => {
                        sleep(running.chaser.not_rotated_wait);
                        continue 'reading;
                    }
                    Err(e) => {
                        if running
                            .chaser
                            .rotation_check_attempts
                            .map(|max_attempts| rotation_check_attempts < max_attempts)
                            .unwrap_or(true)
                        {
                            rotation_check_attempts += 1;
                            sleep(running.chaser.rotation_check_wait);
                            continue 'rotation_check;
                        } else {
                            return Err(e.into());
                        }
                    }
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

#[cfg(any(target_os = "redox", unix))]
fn get_file_id(file: &File) -> Result<FileId, io::Error> {
    let meta = file.metadata()?;
    Ok(FileId(meta.ino()))
}

#[cfg(any(target_os = "linux", target_os = "l4re"))]
fn get_file_id(file: &File) -> Result<FileId, io::Error> {
    let meta = file.metadata()?;
    Ok(FileId(meta.st_ino()))
}

enum RotationStatus {
    Rotated { file: File, file_id: FileId },
    NotRotated,
}
