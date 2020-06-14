use crate::debug;
use crate::ResultExt;
use std::path::Path;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpError {
    #[error("Could not copy {src} to {dst}")]
    CouldNotCopy {
        src: PathBuf,
        dst: PathBuf,
        source: std::io::Error,
    },
}

#[inline(always)]
pub fn cp(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    try_cp(src, dst).or_panic();
}

#[inline(always)]
pub fn mb_cp(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Option<()> {
    try_cp(src, dst).ok_or_log_warn()
}

#[inline(always)]
pub fn try_cp(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), CpError> {
    imp(src.as_ref(), dst.as_ref())
}

fn imp(src: &Path, dst: &Path) -> Result<(), CpError> {
    std::fs::copy(src, dst)
        .map_err(|source| CpError::CouldNotCopy {
            src: src.into(),
            dst: dst.into(),
            source,
        })
        .map(|_| {
            debug!("Copied {} -> {}", src.display(), dst.display());
        })
}
