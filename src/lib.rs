//! Semihosting for ARM Cortex-M processors
//!
//! # What is semihosting?
//!
//! "Semihosting is a mechanism that enables code running on an ARM target to communicate and use
//! the Input/Output facilities on a host computer that is running a debugger." - ARM
//!
//! # Interface
//!
//! Since semihosting operations are modeled as [system calls][sc], this crate exposes an untyped
//! `syscall!` interface just like the [`sc`] crate does.
//!
//! [sc]: https://en.wikipedia.org/wiki/System_call
//! [`sc`]: https://crates.io/crates/sc
//!
//! # Forewarning
//!
//! Semihosting operations are *very* slow. Like, each WRITE operation can take hundreds of
//! milliseconds.
//!
//! # Requirements
//!
//! Compiling this crate on stable or beta requires `arm-none-eabi-gcc` to be installed and
//! available in `PATH`.
//!
//! # Example
//!
//! This example will show how to print "Hello, world!" on the host.
//!
//! Target program:
//!
//! ```
//! #[macro_use]
//! extern crate cortex_m_semihosting;
//!
//! // This function will be called by the application
//! fn print() {
//!     // File descriptor (on the host)
//!     const STDOUT: usize = 1; // NOTE the host stdout may not always be fd 1
//!     static MSG: &'static [u8] = b"Hello, world!\n";
//!
//!     // Signature: fn write(fd: usize, ptr: *const u8, len: usize) -> usize
//!     let r = unsafe { syscall!(WRITE, STDOUT, MSG.as_ptr(), MSG.len()) };
//! }
//! ```
//!
//! On the host side:
//!
//! ``` text
//! $ openocd -f $INTERFACE -f $TARGET -l /tmp/openocd.log
//! Open On-Chip Debugger 0.9.0 (2016-04-27-23:18)
//! Licensed under GNU GPL v2
//! For bug reports, read
//!         http://openocd.org/doc/doxygen/bugs.html
//! # the command will block at this point
//! ```
//!
//! The OpenOCD logs will be redirected to `/tmp/openocd.log`. You can view those logs in "real
//! time" using `tail`
//!
//! ``` text
//! $ tail -f /tmp/openocd.log
//! Info : Unable to match requested speed 1000 kHz, using 950 kHz
//! Info : Unable to match requested speed 1000 kHz, using 950 kHz
//! Info : clock speed 950 kHz
//! Info : STLINK v1 JTAG v11 API v2 SWIM v0 VID 0x0483 PID 0x3744
//! Info : using stlink api v2
//! Info : nrf51.cpu: hardware has 4 breakpoints, 2 watchpoints
//! ```
//!
//! Alternatively you could omit the `-l` flag from the `openocd` call, and the `tail -f` command
//! but the OpenOCD output will have intermingled in it logs from its normal operation.
//!
//! Then, we run the program:
//!
//! ``` text
//! $ arm-none-eabi-gdb hello-world
//! (gdb) # Connect to OpenOCD
//! (gdb) target remote :3333
//!
//! (gdb) # Enable OpenOCD's semihosting support
//! (gdb) monitor arm semihosting enable
//!
//! (gdb) # Flash the program
//! (gdb) load
//!
//! (gdb) # Run the program
//! (gdb) continue
//! ```
//!
//! And you'll see the output under OpenOCD's terminal
//!
//! ``` text
//! # openocd -f $INTERFACE -f $TARGET -l /tmp/openocd.log
//! (..)
//! Hello, world!
//! ```
//!
//! # Optional features
//!
//! ## `inline-asm`
//!
//! When this feature is enabled semihosting is implemented using inline assembly (`asm!`) and
//! compiling this crate requires nightly.
//!
//! When this feature is disabled semihosting is implemented using FFI calls into an external
//! assembly file and compiling this crate works on stable and beta.
//!
//! Apart from the toolchain requirement, disabling `inline-asm` requires `arm-none-eabi-gcc` to be
//! installed on the host. Also, disabling `inline-asm` imposes an overhead of an extra function
//! call on each semihosting call compared to having `inline-asm` enabled.
//!
//! # Reference
//!
//! For documentation about the semihosting operations, check:
//!
//! 'Chapter 8 - Semihosting' of the ['ARM Compiler toolchain Version 5.0'][pdf]
//! manual.
//!
//! [pdf]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0471e/DUI0471E_developing_for_arm_processors.pdf

#![cfg_attr(feature = "inline-asm", feature(asm))]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

#[macro_use]
mod macros;

pub mod debug;
pub mod hio;
pub mod nr;

#[cfg(all(thumb, not(feature = "inline-asm")))]
extern "C" {
    fn __syscall(nr: usize, arg: usize) -> usize;
}

/// Performs a semihosting operation, takes a pointer to an argument block
#[inline(always)]
pub unsafe fn syscall<T>(nr: usize, arg: &T) -> usize {
    syscall1(nr, arg as *const T as usize)
}

/// Performs a semihosting operation, takes one integer as an argument
#[inline(always)]
pub unsafe fn syscall1(_nr: usize, _arg: usize) -> usize {
    match () {
        #[cfg(all(thumb, not(feature = "inline-asm")))]
        () => __syscall(_nr, _arg),

        #[cfg(all(thumb, feature = "inline-asm"))]
        () => {
            let mut nr = _nr;
            asm!("bkpt 0xAB" : "+{r0}"(nr) : "{r1}"(_arg) :: "volatile");
            nr
        }

        #[cfg(not(thumb))]
        () => unimplemented!(),
    }
}
