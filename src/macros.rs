#[macro_export]
macro_rules! error_exit {
    ($($arg:tt)*) => {{
        eprintln!("Error: {}", format!($($arg)*));
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        eprintln!("Warning: {}", format!($($arg)*));
    }};
}
