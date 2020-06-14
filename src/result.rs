use crate::ErrorExt;
use std::error::Error;

pub trait ResultExt<T>: Sized {
    fn on_err_log_warn_and_ignore(self);
    fn on_err_log_error_and_ignore(self);
    fn ok_or_log_warn(self) -> Option<T>;
    fn ok_or_log_error(self) -> Option<T>;
    fn or_panic(self) -> T;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Error,
{
    fn on_err_log_warn_and_ignore(self) {
        if let Err(e) = self {
            e.log_warn();
        }
    }

    fn on_err_log_error_and_ignore(self) {
        if let Err(e) = self {
            e.log_error();
        }
    }

    fn ok_or_log_warn(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(e) => {
                e.log_warn();
                None
            }
        }
    }

    fn ok_or_log_error(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(e) => {
                e.log_error();
                None
            }
        }
    }

    fn or_panic(self) -> T {
        match self {
            Ok(item) => item,
            Err(e) => {
                e.log_error();
                panic!("Unrecoverable error")
            }
        }
    }
}
