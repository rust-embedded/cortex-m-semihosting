/// Variable argument version of `syscall`
#[macro_export]
macro_rules! syscall {
    ($nr:ident) => {
        $crate::syscall1($crate::nr::$nr, 0)
    };
    ($nr:ident, $a1:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize,
                                           $a3 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize,
                                           $a3 as usize, $a4 as usize])
    };
}

/// Macro version of `syscall1`
#[macro_export]
macro_rules! syscall1 {
    ($nr:ident, $a1:expr) => {
        $crate::syscall1($crate::nr::$nr, $a1 as usize)
    };
}

/// Macro for printing to the HOST standard output
///
/// This macro returns a `Result<(), ()>` value
#[macro_export]
macro_rules! hprint {
    ($s:expr) => {
        $crate::export::hstdout_str($s)
    };
    ($($tt:tt)*) => {
        $crate::export::hstdout_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to the HOST standard output, with a newline.
///
/// This macro returns a `Result<(), ()>` value
#[macro_export]
macro_rules! hprintln {
    () => {
        $crate::export::hstdout_str("\n")
    };
    ($s:expr) => {
        $crate::export::hstdout_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::export::hstdout_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}

/// Macro for printing to the HOST standard error
///
/// This macro returns a `Result<(), ()>` value
#[macro_export]
macro_rules! heprint {
    ($s:expr) => {
        $crate::export::hstderr_str($s)
    };
    ($($tt:tt)*) => {
        $crate::export::hstderr_fmt(format_args!($($tt)*))
    };
}

/// Macro for printing to the HOST standard error, with a newline.
///
/// This macro returns a `Result<(), ()>` value
#[macro_export]
macro_rules! heprintln {
    () => {
        $crate::export::hstderr_str("\n")
    };
    ($s:expr) => {
        $crate::export::hstderr_str(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::export::hstderr_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
