#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::error!(target: $target, $($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[E] ");
            eprintln!($target, $($arg)+);
        }
    );
    ($($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::error!($($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[E] ");
            eprintln!($($arg)+);
        }
    )
}
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::warn!(target: $target, $($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[W] ");
            eprintln!($target, $($arg)+);
        }
    );
    ($($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::warn!($($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[W] ");
            eprintln!($($arg)+);
        }
    )
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::info!(target: $target, $($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[I] ");
            eprintln!($target, $($arg)+);
        }
    );
    ($($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::info!($($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[I] ");
            eprintln!($($arg)+);
        }
    )
}
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::debug!(target: $target, $($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[D] ");
            eprintln!($target, $($arg)+);
        }
    );
    ($($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::debug!($($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[D] ");
            eprintln!($($arg)+);
        }
    )
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::trace!(target: $target, $($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[T] ");
            eprintln!($target, $($arg)+);
        }
    );
    ($($arg:tt)+) => (
        #[cfg(feature = "log")]
        log::trace!($($arg)+);
        #[cfg(not(feature = "log"))]
        {
            eprint!("[T] ");
            eprintln!($($arg)+);
        }
    )
}
