use crate::ResultExt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FopenError {
    #[error("Could not open file for read: {path}")]
    CouldNotOpenFile {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[inline(always)]
pub fn fopen(path: impl AsRef<Path>) -> BufReader<File> {
    try_fopen(path).or_panic()
}

#[inline(always)]
pub fn mb_fopen(path: impl AsRef<Path>) -> Option<BufReader<File>> {
    try_fopen(path).ok_or_log_warn()
}

#[inline(always)]
pub fn try_fopen(path: impl AsRef<Path>) -> Result<BufReader<File>, FopenError> {
    imp(path.as_ref())
}

fn imp(path: &Path) -> Result<BufReader<File>, FopenError> {
    File::open(path)
        .map_err(|source| FopenError::CouldNotOpenFile {
            path: path.into(),
            source,
        })
        .map(BufReader::new)
}
