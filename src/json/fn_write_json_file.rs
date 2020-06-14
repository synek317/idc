use crate::fs::{try_fcreate, FCreateError};
use crate::ResultExt;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteJsonError {
    #[error("Could not write JSON to file: {path}")]
    SerializationError {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error(transparent)]
    IO(FCreateError),
}

#[inline(always)]
pub fn write_json_file<T: Serialize>(path: impl AsRef<Path>, content: &T) {
    try_write_json_file(path, content).or_panic()
}

#[inline(always)]
pub fn mb_write_json_file<T: Serialize>(path: impl AsRef<Path>, content: &T) -> Option<()> {
    try_write_json_file(path, content).ok_or_log_warn()
}

#[inline(always)]
pub fn try_write_json_file<T: Serialize>(
    path: impl AsRef<Path>,
    content: &T,
) -> Result<(), WriteJsonError> {
    imp(path.as_ref(), content)
}

fn imp<T: Serialize>(path: &Path, content: &T) -> Result<(), WriteJsonError> {
    try_fcreate(path).map_err(WriteJsonError::IO).and_then(|f| {
        serde_json::ser::to_writer(f, content).map_err(|source| {
            WriteJsonError::SerializationError {
                path: path.into(),
                source,
            }
        })
    })
}
