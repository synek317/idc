// use std::fmt::{self, Display};
use crate::ResultExt;
use std::path::Path;

use crate::fs::{try_fopen, FopenError};
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use thiserror::Error;
// use std::io::{Read, IoSliceMut};

#[derive(Error, Debug)]
pub enum ParseXmlError {
    #[error("Invalid XML in file: {path}")]
    InvalidContent {
        path: PathBuf,
        source: serde_path_to_error::Error<serde_xml_rs::Error>,
    },

    #[error(transparent)]
    IO(FopenError),
}

// #[derive(Error, Debug)]
// pub enum InnerError {
//     #[error("error {info}, path: {path}")]
//     Info {
//         info: Info,
//         path: serde_path_to_error::Path,
//         source: SerdeXmlError,
//     }
// }

// #[derive(Error, Debug)]
// pub enum SerdeXmlError {
//     #[error("{original}")]
//     Inner {
//         original: serde_xml_rs::Error
//     }
// }

#[inline(always)]
pub fn parse_xml_file<T>(path: impl AsRef<Path>) -> T
where
    T: DeserializeOwned,
{
    try_parse_xml_file(path).or_panic()
}

#[inline(always)]
pub fn mb_parse_xml_file<T>(path: impl AsRef<Path>) -> Option<T>
where
    T: DeserializeOwned,
{
    try_parse_xml_file(path).ok_or_log_warn()
}

#[inline(always)]
pub fn try_parse_xml_file<T>(path: impl AsRef<Path>) -> Result<T, ParseXmlError>
where
    T: DeserializeOwned,
{
    imp(path.as_ref())
}

// #[derive(Copy, Clone, Debug)]
// pub struct Info {
//     line_from: usize,
//     line_to: usize,
//     pos_from: usize,
//     pos_to: usize,
// }

// impl Default for Info {
//     fn default() -> Self {
//         Self {
//             line_from: 1,
//             line_to: 1,
//             pos_from: 0,
//             pos_to: 0   ,
//         }
//     }
// }

// impl Display for Info {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self.line_from == self.line_to {
//             if self.pos_from == self.pos_to {
//                 return write!(f, "at line {}, byte {}", self.line_from, self.pos_from);
//             }

//             return write!(f, "at line {}, around bytes {} - {}", self.line_from, self.pos_from, self.pos_to )
//         }

//         if self.pos_from == self.pos_to {
//             return write!(f, "around lines {} - {}, at byte {}", self.line_from, self.line_to, self.pos_from);
//         }

//         write!(f, "around lines {} - {}, bytes {} - {}", self.line_from, self.line_to, self.pos_from, self.pos_to )
//     }
// }

// struct LineAwareReader<R: Read> {
//     info: Info,
//     inner: R
// }

// impl<R: Read> LineAwareReader<R> {
//     pub fn new(inner: R) -> Self {
//         Self {
//             info: Default::default(),
//             inner,
//         }
//     }

//     fn update_info<'a>(&mut self, read_result: &std::io::Result<usize>, read_bytes: impl Iterator<Item = &'a u8>) {
//         if let Ok(n) = read_result {
//             self.info.line_from = self.info.line_to;
//             self.info.pos_from = self.info.pos_to + 1;
//             self.info.pos_to = self.info.pos_from + n - 1;
//             self.info.line_to = self.info.line_from + read_bytes.filter(|b| **b == b'\n').count();
//         }
//     }
// }

// impl<R: Read> Read for LineAwareReader<R> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
//     {
//         let res = self.inner.read(buf);

//         self.update_info(&res, buf.iter());

//         res
//     }

//     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
//         let res = self.inner.read_vectored(bufs);

//         self.update_info(&res, bufs.iter().map(|s| s.iter()).flatten());

//         res
//     }
// }

fn imp<T>(path: &Path) -> Result<T, ParseXmlError>
where
    T: DeserializeOwned,
{
    //    ulepszyć error:
    //        - nakładką na bufreader
    //        - serde error path (taki crate)
    try_fopen(path)
        .map_err(ParseXmlError::IO)
        //        .map(LineAwareReader::new)
        .and_then(|mut reader| {
            let deserializer = &mut serde_xml_rs::Deserializer::new_from_reader(&mut reader);

            // serde_xml_rs::from_reader(reader)
            //     .map_err(|source| ParseXmlError::InvalidContent { path: path.into(), source })
            serde_path_to_error::deserialize(deserializer)
                //                .map_err(|source| InnerError::Info {
                //                    path: source.path().clone(),
                //                    info: reader.info,
                //                    source: SerdeXmlError::Inner { original: source.into_inner() },
                //                })
                //                .map_err(|source| ParseXmlError::InvalidContent { path: path.into(), source })
                //                .map_err(|source| ParseXmlError::InvalidContent { file_path: path.into(), error_path: source.path().clone(), source: source.into_inner() })
                .map_err(|source| ParseXmlError::InvalidContent {
                    path: path.into(),
                    source,
                })
        })
    //        .map(LineAwareReader::new)
    //        .and_then(|mut reader| {
    //            let deserializer = &mut serde_xml_rs::Deserializer::new_from_reader(&mut reader);
    //
    //            serde_path_to_error::deserialize(deserializer)
    //                .map_err(|source| InnerError::Info {
    //                    path: source.path().clone(),
    //                    info: reader.info,
    //                    source: SerdeXmlError::Inner { original: source.into_inner() },
    //                })
    //                .map_err(|source| ParseXmlError::InvalidContent { path: path.into(), source })
    //        })
}
