use crate::ResultExt;
use thiserror::Error;
use std::any::type_name;
use std::fmt::Debug;
use std::error::Error as StdError;
use std::str::FromStr;

#[derive(Error, Debug)]
enum Error<'a, S>
  where S: StdError + Debug + 'static
{
    #[error("Could not parse '{input}' to {typ}")]
    InvalidInput {
        input: &'a str,
        typ: &'static str,
        source: S
    }
}

pub fn pu<T>(input: &str) -> T
    where T: FromStr,
          <T as FromStr>::Err: StdError + 'static
{
    input.parse()
      .map_err(|source| Error::InvalidInput { input, source, typ: type_name::<T>() })
      .or_panic()

}

pub trait ParseUnwrap {
    fn pu<T>(&self) -> T
    where T: FromStr,
          <T as FromStr>::Err: StdError + 'static;

}

impl ParseUnwrap for str {
    fn pu<T>(&self) -> T
    where T: FromStr,
          <T as FromStr>::Err: StdError + 'static
    {
      pu(self)
    }

}