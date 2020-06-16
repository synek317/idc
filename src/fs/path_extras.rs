use std::path::Path;
use thiserror::Error;
use crate::ResultExt;
use std::ffi::OsStr;

pub fn is_dir<T: AsRef<Path> + ?Sized>(path: &T) -> bool {
    path.as_ref().is_dir()
}

pub fn is_file<T: AsRef<Path> + ?Sized>(path: &T) -> bool {
    path.as_ref().is_file()
}

pub fn fstem<T: AsRef<Path> + ?Sized>(path: &T) -> &OsStr {
    let p = path.as_ref();

    p.file_stem().ok_or(Error::NoStem(p)).or_panic()
}

pub fn fstem_str<T: AsRef<Path> + ?Sized>(path: &T) -> &str {
    let p = path.as_ref();

    fstem(path).to_str().ok_or(Error::NonUtf8Path(p)).or_panic()
}

pub fn fstem_s<T: AsRef<Path> + ?Sized>(path: &T) -> String {
    fstem(path).to_string_lossy().to_string()
}

pub trait PathExt {
    fn fstem(&self) -> &OsStr;
    fn fstem_str(&self) -> &str;
    fn fstem_s(&self) -> String;
}

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Path has no file stem: {0}")]
    NoStem(&'a Path),

    #[error("Non-UTF8 path: {0}")]
    NonUtf8Path(&'a Path)
}

impl<T> PathExt for T where T: AsRef<Path> {
    fn fstem(&self) -> &OsStr {
        fstem(self)
    }

    fn fstem_str(&self) -> &str {
      fstem_str(self)
    }

    fn fstem_s(&self) -> String {
      fstem_s(self)
    }
}