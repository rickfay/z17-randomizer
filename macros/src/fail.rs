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
        cli::pause();
        std::process::exit(1);
    });
    ($($arg:tt)+) => ({
        log::error!($($arg)+);
        cli::pause();
        std::process::exit(1);
    });
    () => ({
        cli::pause();
        std::process::exit(1);
    });
}
