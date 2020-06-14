use crate::ResultExt;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FAppendError {
    #[error("Could not open file for write: {path}")]
    CouldNotOpenFile {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[inline(always)]
pub fn fappend(path: impl AsRef<Path>) -> BufWriter<File> {
    try_fappend(path).or_panic()
}

#[inline(always)]
pub fn mb_fappend(path: impl AsRef<Path>) -> Option<BufWriter<File>> {
    try_fappend(path).ok_or_log_warn()
}

#[inline(always)]
pub fn try_fappend(path: impl AsRef<Path>) -> Result<BufWriter<File>, FAppendError> {
    imp(path.as_ref())
}

fn imp(path: &Path) -> Result<BufWriter<File>, FAppendError> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|source| FAppendError::CouldNotOpenFile {
            path: path.into(),
            source,
        })
        .map(BufWriter::new)
}
