use crate::debug;
use crate::ResultExt;
use std::path::Path;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MkdirpError {
    #[error("Could not create directory or one of it's parents: {path}")]
    CouldNotCreateDirectory {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[inline(always)]
pub fn mkdir_p(path: impl AsRef<Path>) {
    try_mkdir_p(path).or_panic();
}

#[inline(always)]
pub fn mb_mkdir_p(path: impl AsRef<Path>) -> Option<()> {
    try_mkdir_p(path).ok_or_log_warn()
}

#[inline(always)]
pub fn try_mkdir_p(path: impl AsRef<Path>) -> Result<(), MkdirpError> {
    imp(path.as_ref())
}

fn imp(path: &Path) -> Result<(), MkdirpError> {
    std::fs::create_dir_all(path)
        .map_err(|e| MkdirpError::CouldNotCreateDirectory {
            path: path.into(),
            source: e,
        })
        .map(|ok| {
            debug!("Created directory {}", path.display());
            ok
        })
}
