use std::path::{PathBuf, Path};
use std::io::{self, Read};
use std::fs::{self, File};
use failure;
use std::str::FromStr;
use std::num::ParseIntError;

mod major;
pub use self::major::Major;

#[derive(Debug)]
pub struct DeviceNumber {
    pub major: Major,
    pub minor: u16,
}

impl FromStr for DeviceNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(":").collect();

        Ok(DeviceNumber {
            major: coords[0].parse::<u32>()?.into(),
            minor: coords[1].parse::<u16>()?,
        })
    }
}

#[derive(Debug)]
pub struct Disk {
    path: PathBuf,
    device_number: DeviceNumber,
    removable: bool,
}

pub struct DiskIter {
    inner: fs::ReadDir,
}

fn read_from<T, P>(file_name: P) -> Result<T, failure::Error>
where
    T: FromStr,
    <T as FromStr>::Err: failure::Fail,
    P: AsRef<Path>,
{
    let mut buffer = String::new();
    File::open(file_name)?.read_to_string(&mut buffer)?;
    Ok(buffer.trim().parse()?)
}

impl Iterator for DiskIter {
    type Item = Result<Disk, failure::Error>;

    fn next(&mut self) -> Option<Result<Disk, failure::Error>> {
        match self.inner.next() {
            Some(Ok(dir)) => Some(Disk::new(dir.path())),
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}

impl Disk {
    pub fn new(path: PathBuf) -> Result<Disk, failure::Error> {
        let removable = read_from::<u8, _>(path.join("removable"))? == 1;
        let device_number = read_from(path.join("dev"))?;
        Ok(Disk {
            path,
            device_number,
            removable,
        })
    }

    pub fn list() -> io::Result<DiskIter> {
        Ok(DiskIter { inner: fs::read_dir("/sys/block")? })
    }

    pub fn is_removable(&self) -> bool {
        self.removable
    }

    pub fn path<'a>(&'a self) -> &'a Path {
        &self.path
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
