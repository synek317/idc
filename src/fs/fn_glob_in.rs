use std::path::PathBuf;
use crate::ResultExt;
use super::GlobError;
use super::glob;

#[inline(always)]
pub fn glob_in<'a>(root: impl AsRef<Path>, pattern: &'a str) -> impl Iterator<Item = PathBuf> + 'a {
    try_glob_in(root, pattern).or_panic()
}

#[inline(always)]
pub fn mb_glob_in<'a>(root: impl AsRef<Path>, pattern: &'a str) -> Option<impl Iterator<Item = PathBuf> + 'a> {
    try_glob_in(root, pattern).ok_or_log_warn()
}

#[inline(always)]
pub fn try_glob_in<'a>(root: impl AsRef<Path>, pattern: &'a str) -> Result<impl Iterator<Item = PathBuf> + 'a, GlobError> {
    glob(&format!("{}/{}", root.as_ref().display(), pattern)
}