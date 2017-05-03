//! Host I/O

use core::{fmt, slice};
use core::fmt::Write;
use core::ptr;

/// File descriptors
static mut STDOUT: isize = -1;
static mut STDERR: isize = -1;

/// Host's standard error
struct Stderr;

/// Host's standard output
struct Stdout;

pub fn get_stdout() -> isize {
    // Safe: 32-bit accesses are atomic on ARM
    unsafe{ptr::read_volatile(&STDOUT)}
}

pub fn get_stderr() -> isize {
    // Safe: 32-bit accesses are atomic on ARM
    unsafe{ptr::read_volatile(&STDERR)}
}

/// Open stdout and stderr.
pub fn open_streams() -> Result<(),()> {
    // Special terminal path
    let path = ":tt";

    // To open stdin, use flag 0 instead of 4 or 8
    let stdout_fd = unsafe { syscall!(OPEN, path.as_bytes().as_ptr(), 4, path.len()) } as isize;
    let stderr_fd = unsafe { syscall!(OPEN, path.as_bytes().as_ptr(), 8, path.len()) } as isize;

    // Safe: 32-bit accesses are atomic on ARM
    unsafe {
        ptr::write_volatile(&mut STDOUT, stdout_fd);
        ptr::write_volatile(&mut STDERR, stderr_fd);
    }

    if stdout_fd < 0 || stderr_fd < 0 {
        Err(())
    } else {
        Ok(())
    }
}

/// Write the contents of `buffer` to `fd`. If `fd` is less than zero, do nothing and return
/// `Err(())`.
pub fn write_all(fd: isize, mut buffer: &[u8]) -> Result<(),()> {
    if fd < 0 {
        return Err(());
    }

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
        write_all(get_stderr(), buffer)
    }
}

impl Stdout {
    fn write_all(&mut self, buffer: &[u8]) -> Result<(), ()> {
        write_all(get_stdout(), buffer)
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
