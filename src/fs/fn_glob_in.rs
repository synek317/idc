use std::path::{Path, PathBuf};
use crate::ResultExt;
use super::GlobError;
use thiserror::Error;

#[inline(always)]
pub fn glob_in<'a>(root: impl AsRef<Path> + 'a, pattern: &'a str) -> impl Iterator<Item = PathBuf> + 'a {
    try_glob_in(root, pattern).or_panic()
}

#[inline(always)]
pub fn mb_glob_in<'a>(root: impl AsRef<Path> + 'a, pattern: &'a str) -> Option<impl Iterator<Item = PathBuf> + 'a> {
    try_glob_in(root, pattern).ok_or_log_warn()
}

#[inline(always)]
pub fn try_glob_in<'a>(root: impl AsRef<Path> + 'a, pattern: &'a str) -> Result<impl Iterator<Item = PathBuf> + 'a, GlobError> {
    #[derive(Error, Debug)]
    pub enum InnerError {
        #[error("Unreadable path while iterating over results of glob: {pattern}")]
        InvalidGlobPattern {
            pattern: String,
            source: ::glob::GlobError,
        },
    }

    let pattern = format!("{}/{}", root.as_ref().display(), pattern);

    ::glob::glob(&pattern)
        .map_err(|source| GlobError::InvalidGlobPattern {
            pattern: pattern.to_string(),
            source,
        })
        .map(move |ok| {
            ok.filter_map(move |r| {
                r.map_err(|source| InnerError::InvalidGlobPattern {
                    pattern: pattern.to_string(),
                    source,
                })
                .ok_or_log_warn()
            })
        })
}