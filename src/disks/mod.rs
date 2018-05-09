use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::io::{self, Read};
use std::fs::{self, File};
use failure;
use std::str::FromStr;
use std::num::ParseIntError;

mod major;
pub use self::major::Major;

#[derive(Debug, Copy, Clone)]
pub struct DeviceNumber {
    pub major: Major,
    pub minor: u16,
}
#[derive(Debug, Clone)]
pub struct Device {
    pub model: String,
}

#[derive(Debug)]
pub struct BlockDevice {
    dev: OsString,
    device_number: DeviceNumber,
    removable: bool,
    device: Option<Device>,
    size: Size,
}

#[derive(Debug, Copy, Clone)]
pub struct Size(pub u64);

pub struct BlockDeviceIter {
    inner: fs::ReadDir,
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

impl FromStr for Size {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Size(s.parse::<u64>()?))
    }
}

use std::fmt;

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.0 * 512;
        match size {
            0...1024 => write!(f, "{}", size),
            1024...1_048_576 => write!(f, "{:.1}KiB", size as f64 / 1024.0),
            1_048_576...1_073_741_824 => write!(f, "{:.1}MiB", size as f64 / 1_048_576.0),
            1_073_741_824...1_099_511_627_776 => {
                write!(f, "{:.1}GiB", size as f64 / 1_073_741_824.0)
            }
            _ => write!(f, "{:.1}TiB", self.0),
        }
    }
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

impl Iterator for BlockDeviceIter {
    type Item = Result<BlockDevice, failure::Error>;

    fn next(&mut self) -> Option<Result<BlockDevice, failure::Error>> {
        match self.inner.next() {
            Some(Ok(dir)) => Some(BlockDevice::new(dir.path().file_name().unwrap().into())),
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}

impl BlockDevice {
    pub fn new(dev: OsString) -> Result<BlockDevice, failure::Error> {
        let path = PathBuf::from("/sys/block").join(dev.clone());

        Ok(BlockDevice {
            dev,
            device_number: read_from(path.join("dev"))?,
            removable: read_from::<u8, _>(path.join("removable"))? == 1,
            device: {
                let device_path = path.join("device");
                if device_path.exists() {
                    Some(Device {
                        model: read_from(device_path.join("model"))?,
                    })
                } else {
                    None
                }
            },
            size: read_from::<Size, _>(path.join("size"))?,
        })
    }

    pub fn list() -> io::Result<BlockDeviceIter> {
        Ok(BlockDeviceIter {
            inner: fs::read_dir("/sys/block")?,
        })
    }

    pub fn device_number(&self) -> DeviceNumber {
        self.device_number
    }

    pub fn is_removable(&self) -> bool {
        self.removable
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from("/dev").join(self.dev.clone())
    }

    pub fn device(&self) -> Option<Device> {
        self.device.clone()
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_disks() {
        for disk in BlockDevice::list().unwrap() {
            println!("{:?}", disk);
        }
    }
}
