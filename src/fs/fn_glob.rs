use crate::ResultExt;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GlobError {
    #[error("Invalid glob pattern: {pattern}")]
    InvalidGlobPattern {
        pattern: String,
        source: ::glob::PatternError,
    },
}

#[inline(always)]
pub fn glob<'a>(pattern: &'a str) -> impl Iterator<Item = PathBuf> + 'a {
    try_glob(pattern).or_panic()
}

#[inline(always)]
pub fn mb_glob<'a>(pattern: &'a str) -> Option<impl Iterator<Item = PathBuf> + 'a> {
    try_glob(pattern).ok_or_log_warn()
}

#[inline(always)]
pub fn try_glob<'a>(pattern: &'a str) -> Result<impl Iterator<Item = PathBuf> + 'a, GlobError> {
    imp(pattern)
}

fn imp<'a>(pattern: &'a str) -> Result<impl Iterator<Item = PathBuf> + 'a, GlobError> {
    #[derive(Error, Debug)]
    pub enum InnerError {
        #[error("Unreadable path while iterating over results of glob: {pattern}")]
        InvalidGlobPattern {
            pattern: String,
            source: ::glob::GlobError,
        },
    }

    ::glob::glob(pattern)
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
