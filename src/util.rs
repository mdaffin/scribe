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
