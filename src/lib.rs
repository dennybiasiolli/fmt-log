//! console_log
//!
//! This module contains macros for logging to the console
//! using the `std::println!` or `std::eprintln!` macros, but also returns the
//! formatted string.

/// Logs to the console using `std::println!` macro
/// and returns the formatted string.
///
/// # Examples
///
/// ```rust
/// use console_log::console_log;
///
/// let s1 = "Hello";
/// let s2 = String::from("world!");
/// let n1 = 123;
/// let output = console_log!("{}, {} {}", s1, s2, n1);
/// assert_eq!(output, format!("{}, {} {}", s1, s2, n1));
/// ```
/// This will log "Hello, world!" to the console.
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        {
            std::println!($($arg)*);
            std::format!($($arg)*)
        }
    }
}

/// Logs to the console using `std::eprintln!` macro
/// and returns the formatted string.
///
/// # Examples
///
/// ```rust
/// use console_log::console_log;
///
/// let s1 = "Hello";
/// let s2 = String::from("world!");
/// let n1 = 123;
/// let output = console_log!("{}, {} {}", s1, s2, n1);
/// assert_eq!(output, format!("{}, {} {}", s1, s2, n1));
/// ```
/// This will log "Hello, world!" to the console.
#[macro_export]
macro_rules! console_error {
    ($($arg:tt)*) => {
        {
            std::eprintln!($($arg)*);
            std::format!($($arg)*)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_logs_to_log() {
        let s1 = "Hello";
        let s2 = String::from("world!");
        let n1 = 123;
        let output = console_log!("{}, {} {}", s1, s2, n1);
        assert_eq!(output, format!("{}, {} {}", s1, s2, n1));
        // Hello, world! 123
    }

    #[test]
    fn it_logs_to_error() {
        let s1 = "Hello";
        let s2 = String::from("world!");
        let n1 = 123;
        let output = console_error!("{}, {} {}", s1, s2, n1);
        assert_eq!(output, format!("{}, {} {}", s1, s2, n1));
        // Hello, world! 123
    }
}
