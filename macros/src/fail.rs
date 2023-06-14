use std::io::{stdin, stdout, Read, Write};

/// Pauses program execution
pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...\n").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

/**
 * Shuts down the program in a controlled fashion:
 * - Displays an error message (optional)
 * - Pauses execution of the CLI
 * - Terminates with exit code 1.
 */
#[macro_export]
macro_rules! fail {
    (target: $target:expr, $($arg:tt)+) => ({
        log::error!(target: $target, $($arg)+);
        macros::fail::pause();
        std::process::exit(1);
    });
    ($($arg:tt)+) => ({
        log::error!($($arg)+);
        macros::fail::pause();
        std::process::exit(1);
    });
    () => ({
        macros::fail::pause();
        std::process::exit(1);
    });
}
