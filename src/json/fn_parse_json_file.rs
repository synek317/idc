use crate::ResultExt;
use std::fmt::{self, Display};
use std::path::Path;

use crate::fs::{try_fopen, FopenError};
use serde::de::DeserializeOwned;
use std::io::{IoSliceMut, Read};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseJsonError {
    #[error("Invalid JSON file: {path}")]
    InvalidContent { path: PathBuf, source: InnerError },

    #[error(transparent)]
    IO(FopenError),
}

#[derive(Error, Debug)]
pub enum InnerError {
    #[error("error {info}, path: {path}")]
    Info {
        info: Info,
        path: serde_path_to_error::Path,
        source: SerdeJsonError,
    },
}

#[derive(Error, Debug)]
pub enum SerdeJsonError {
    #[error("{original}")]
    Inner { original: serde_json::Error },
}

#[inline(always)]
pub fn parse_json_file<T>(path: impl AsRef<Path>) -> T
where
    T: DeserializeOwned,
{
    try_parse_json_file(path).or_panic()
}

#[inline(always)]
pub fn mb_parse_json_file<T>(path: impl AsRef<Path>) -> Option<T>
where
    T: DeserializeOwned,
{
    try_parse_json_file(path).ok_or_log_warn()
}

#[inline(always)]
pub fn try_parse_json_file<T>(path: impl AsRef<Path>) -> Result<T, ParseJsonError>
where
    T: DeserializeOwned,
{
    imp(path.as_ref())
}

#[derive(Copy, Clone, Debug)]
pub struct Info {
    line_from: usize,
    line_to: usize,
    pos_from: usize,
    pos_to: usize,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            line_from: 1,
            line_to: 1,
            pos_from: 0,
            pos_to: 0,
        }
    }
}

impl Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line_from == self.line_to {
            if self.pos_from == self.pos_to {
                return write!(f, "at line {}, byte {}", self.line_from, self.pos_from);
            }

            return write!(
                f,
                "at line {}, around bytes {} - {}",
                self.line_from, self.pos_from, self.pos_to
            );
        }

        if self.pos_from == self.pos_to {
            return write!(
                f,
                "around lines {} - {}, at byte {}",
                self.line_from, self.line_to, self.pos_from
            );
        }

        write!(
            f,
            "around lines {} - {}, bytes {} - {}",
            self.line_from, self.line_to, self.pos_from, self.pos_to
        )
    }
}

struct LineAwareReader<R: Read> {
    info: Info,
    inner: R,
}

impl<R: Read> LineAwareReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            info: Default::default(),
            inner,
        }
    }

    fn update_info<'a>(
        &mut self,
        read_result: &std::io::Result<usize>,
        read_bytes: impl Iterator<Item = &'a u8>,
    ) {
        if let Ok(n) = read_result {
            self.info.line_from = self.info.line_to;
            self.info.pos_from = self.info.pos_to + 1;
            self.info.pos_to = self.info.pos_from + n - 1;
            self.info.line_to = self.info.line_from + read_bytes.filter(|b| **b == b'\n').count();
        }
    }
}

impl<R: Read> Read for LineAwareReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.inner.read(buf);

        self.update_info(&res, buf.iter());

        res
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
        let res = self.inner.read_vectored(bufs);

        self.update_info(&res, bufs.iter().map(|s| s.iter()).flatten());

        res
    }
}

fn imp<T>(path: &Path) -> Result<T, ParseJsonError>
where
    T: DeserializeOwned,
{
    try_fopen(path)
        .map_err(ParseJsonError::IO)
        .map(LineAwareReader::new)
        .and_then(|mut reader| {
            let deserializer = &mut serde_json::Deserializer::from_reader(&mut reader);

            serde_path_to_error::deserialize(deserializer)
                .map_err(|source| InnerError::Info {
                    path: source.path().clone(),
                    info: reader.info,
                    source: SerdeJsonError::Inner {
                        original: source.into_inner(),
                    },
                })
                .map_err(|source| ParseJsonError::InvalidContent {
                    path: path.into(),
                    source,
                })
        })
}
