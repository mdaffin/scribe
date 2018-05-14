use std::fmt;
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::io;
use std::path::Path;
use std::str::FromStr;

/// A wrapper around a bool to allow parsing from strings of `1` or `0`.
///
/// Example:
///
/// ```
/// assert_eq!("0".parse::<IntBool>().unwrap() as bool, false);
/// assert_eq!("1".parse::<IntBool>().unwrap() as bool, true);
/// ```
#[derive(Debug)]
pub struct IntBool(bool);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIntBoolError {
    _priv: (),
}

/// Converts a Result<T> to a Result<Option<T>> where Ok(None) is returned if the error was
/// std::io::ErrorKind::NotFound
macro_rules! if_exists {
    ($x:expr) => {
        match $x {
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Ok(c) => Ok(Some(c)),
            Err(e) => Err(e),
        }
    };
}

/// Reads the given file and parses it into type T.
pub fn read_to<P, T>(file_name: P) -> Result<T, io::Error>
where
    P: AsRef<Path>,
    <T as FromStr>::Err: Debug,
    T: FromStr + Debug,
{
    match read_to_string(file_name.as_ref()) {
        Ok(contents) => Ok(contents.trim().parse().expect(&format!(
            "could not parse contents of {}",
            file_name.as_ref().display()
        ))),
        Err(e) => Err(e),
    }
}

impl FromStr for IntBool {
    type Err = ParseIntBoolError;

    /// Parse a the string `0` and `1` into a `bool`.
    #[inline]
    fn from_str(s: &str) -> Result<IntBool, ParseIntBoolError> {
        match s {
            "1" => Ok(IntBool(true)),
            "0" => Ok(IntBool(false)),
            _ => Err(ParseIntBoolError { _priv: () }),
        }
    }
}

impl From<IntBool> for bool {
    fn from(b: IntBool) -> Self {
        b.0
    }
}

impl fmt::Display for ParseIntBoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt("provided string was not `0` or `1`", f)
    }
}
