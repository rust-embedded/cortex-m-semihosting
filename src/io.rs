//! Host I/O

use core::{fmt, slice};
use core::fmt::Write;

/// File descriptors
const STDOUT: usize = 1;
const STDERR: usize = 2;

/// Host's standard error
struct Stderr;

/// Host's standard output
struct Stdout;

fn write_all(fd: usize, mut buffer: &[u8]) -> Result<(),()> {
    while !buffer.is_empty() {
        match unsafe { syscall!(WRITE, fd, buffer.as_ptr(), buffer.len()) } {
            // Done
            0 => return Ok(()),
            // `n` bytes were not written
            n if n <= buffer.len() && n > 0 => {
                let offset = (buffer.len() - n) as isize;
                buffer = unsafe {
                    slice::from_raw_parts(buffer.as_ptr().offset(offset as isize), n)
                };
            },
            // error writing bytes, most likely write() returned -1
            _ => return Err(()),
        }
    }

    Ok(())
}

impl Stderr {
    fn write_all(&mut self, buffer: &[u8]) -> Result<(),()> {
        write_all(STDERR, buffer)
    }
}

impl Stdout {
    fn write_all(&mut self, buffer: &[u8]) -> Result<(), ()> {
        write_all(STDOUT, buffer)
    }
}

impl Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).or(Err(fmt::Error))
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).or(Err(fmt::Error))
    }
}

/// Write a `buffer` to the host's stderr
pub fn ewrite(buffer: &[u8]) -> Result<(),()> {
    Stderr.write_all(buffer)
}

/// Write `fmt::Arguments` to the host's stderr
pub fn ewrite_fmt(args: fmt::Arguments) -> fmt::Result {
    Stderr.write_fmt(args)
}

/// Write a `string` to the host's stderr
pub fn ewrite_str(string: &str) -> Result<(),()> {
    Stderr.write_all(string.as_bytes())
}

/// Write a `buffer` to the host's stdout
pub fn write(buffer: &[u8]) ->Result<(),()> {
    Stdout.write_all(buffer)
}

/// Write `fmt::Arguments` to the host's stdout
pub fn write_fmt(args: fmt::Arguments) -> fmt::Result {
    Stdout.write_fmt(args)
}

/// Write a `string` to the host's stdout
pub fn write_str(string: &str) -> Result<(),()> {
    Stdout.write_all(string.as_bytes())
}
