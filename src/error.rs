use crate::{debug, error, info, trace, warn};
use std::error::Error;
use std::io::{self, stderr, stdout, Write};

pub trait ErrorExt {
    fn log_error(&self);
    fn log_warn(&self);
    fn log_info(&self);
    fn log_debug(&self);
    fn log_trace(&self);

    fn write(&self, writer: &mut dyn Write) -> Result<(), io::Error>;

    fn print(&self) -> Result<(), io::Error> {
        self.write(&mut stdout())
    }

    fn eprint(&self) -> Result<(), io::Error> {
        self.write(&mut stderr())
    }
}

impl<T> ErrorExt for T
where
    T: Error,
{
    fn log_error(&self) {
        error!("{}", self);

        let mut e: &dyn std::error::Error = self;

        while let Some(cause) = e.source() {
            error!("caused by: {}", cause);
            e = cause;
        }
    }

    fn log_warn(&self) {
        warn!("{}", self);

        let mut e: &dyn std::error::Error = self;

        while let Some(cause) = e.source() {
            warn!("caused by: {}", cause);
            e = cause;
        }
    }

    fn log_info(&self) {
        info!("{}", self);

        let mut e: &dyn std::error::Error = self;

        while let Some(cause) = e.source() {
            info!("caused by: {}", cause);
            e = cause;
        }
    }

    fn log_debug(&self) {
        debug!("{}", self);

        let mut e: &dyn std::error::Error = self;

        while let Some(cause) = e.source() {
            debug!("caused by: {}", cause);
            e = cause;
        }
    }

    fn log_trace(&self) {
        trace!("{}", self);

        let mut e: &dyn std::error::Error = self;

        while let Some(cause) = e.source() {
            trace!("caused by: {}", cause);
            e = cause;
        }
    }

    fn write(&self, writer: &mut dyn Write) -> Result<(), io::Error> {
        writeln!(writer, "{}", self)?;

        let mut e: &dyn std::error::Error = self;

        while let Some(cause) = e.source() {
            writeln!(writer, "caused by: {}", cause)?;
            e = cause;
        }

        Ok(())
    }
}
