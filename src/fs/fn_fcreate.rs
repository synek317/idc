use crate::ResultExt;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FCreateError {
    #[error("Could not create file: {path}")]
    CouldNotOpenFile {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[inline(always)]
pub fn fcreate(path: impl AsRef<Path>) -> BufWriter<File> {
    try_fcreate(path).or_panic()
}

#[inline(always)]
pub fn mb_fcreate(path: impl AsRef<Path>) -> Option<BufWriter<File>> {
    try_fcreate(path).ok_or_log_warn()
}

#[inline(always)]
pub fn try_fcreate(path: impl AsRef<Path>) -> Result<BufWriter<File>, FCreateError> {
    imp(path.as_ref())
}

fn imp(path: &Path) -> Result<BufWriter<File>, FCreateError> {
    File::create(path)
        .map_err(|source| FCreateError::CouldNotOpenFile {
            path: path.into(),
            source,
        })
        .map(BufWriter::new)
}
