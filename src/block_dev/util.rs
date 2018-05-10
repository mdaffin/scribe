#![feature(fs_read_write)]

use std::io::{self, Read};
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::fmt::Debug;

/// Reads the given file into a string if the file exists, returns Ok(None) if the file does not
/// exist.
pub fn read_into_if_exists<P, T>(file_name: P) -> Result<Option<T>, io::Error>
where
    P: AsRef<Path>,
    <T as FromStr>::Err: Debug,
    T: FromStr + Debug,
{
    match read_to_string(file_name) {
        Ok(contents) => Ok(Some(contents.parse().unwrap())),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn read_into<P, T>(file_name: P) -> Result<T, io::Error>
where
    P: AsRef<Path>,
    <T as FromStr>::Err: Debug,
    T: FromStr + Debug,
{
    match read_to_string(file_name) {
        Ok(contents) => Ok(contents.parse().unwrap()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::fs::write;

    #[test]
    fn test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(file!());
        d.pop();

        println!("{:?}", d);
        assert!(false);
    }
}
