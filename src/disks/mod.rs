use std::path::PathBuf;
use std::io::{self, Read};
use std::fs::{self, DirEntry, File};

#[derive(Debug)]
pub struct Disk {
    dir: DirEntry,
    path: PathBuf,
    removable: bool,
}

pub struct DiskIter {
    inner: fs::ReadDir,
}

macro_rules! read_byte {
    ($file_name:expr) => (
        {
            let file =  match File::open($file_name) {
                Err(err) => return Some(Err(err)),
                Ok(v) => v,
            };
            let path = match
            match file.bytes()
                .next() { 
                Some(Ok(c)) => c,
                Some(Err(err)) => return Some(Err(err)),
                None => return Some(Err(io::ErrorKind::UnexpectedEof.into())),
            }
        }
    )
}


impl Iterator for DiskIter {
    type Item = io::Result<Disk>;

    fn next(&mut self) -> Option<io::Result<Disk>> {
        match self.inner.next() {
            Some(Ok(path)) => {
                let removable = read_byte!(path.path().join("removable")) == '1' as u8;
                Some(Ok(Disk { path, removable }))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

impl Disk {
    pub fn list() -> io::Result<DiskIter> {
        Ok(DiskIter { inner: fs::read_dir("/sys/block")? })
    }

    pub fn is_removable(&self) -> bool {
        self.removable
    }

    pub fn path(&self) -> PathBuf {
        self.path.path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_disks() {
        for disk in Disk::list().unwrap() {
            println!("{:?}", disk);
        }
    }
}
