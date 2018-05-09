use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::io;
use std::fs;
use failure;
use std::str::FromStr;
use std::num::ParseIntError;

mod major;
mod util;
//use self::util::read_from;
pub use self::major::Major;

#[derive(Debug, Copy, Clone)]
pub struct DeviceNumber {
    pub major: Major,
    pub minor: u16,
}
#[derive(Debug, Clone)]
pub struct Device {
    pub model: Option<String>,
    pub vendor: Option<String>,
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

impl Iterator for BlockDeviceIter {
    type Item = Result<BlockDevice, io::Error>;

    fn next(&mut self) -> Option<Result<BlockDevice, io::Error>> {
        match self.inner.next() {
            Some(Ok(dir)) => Some(BlockDevice::new(
                dir.path()
                    .file_name()
                    .expect(&format!("missing file_name for '{}'", dir.path().display()))
                    .into(),
            )),
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}

impl Device {
    pub fn new<P>(device_path: P) -> Result<Device, failure::Error>
    where
        P: AsRef<Path>,
    {
        Ok(Device {
            model: read_to_string(device_path.as_ref().join("model"))
                .map(|v| Some(v))
                .unwrap(),
            vendor: read_to_string(device_path.as_ref().join("vendor"))
                .map(|v| Some(v))
                .unwrap(),
        })
    }
}

impl BlockDevice {
    pub fn new(dev: OsString) -> Result<BlockDevice, io::Error> {
        let path = PathBuf::from("/sys/block").join(dev.clone());

        Ok(BlockDevice {
            dev,
            device_number: read_to_string(path.join("dev"))?
                .parse()
                .expect("failed to parse dev"),
            removable: read_to_string(path.join("removable"))?
                .parse::<u8>()
                .expect("failed to parse removable") == 1,
            device: {
                let device_path = path.join("device");
                if device_path.exists() {
                    Some(Device::new(device_path).unwrap())
                } else {
                    None
                }
            },
            size: read_to_string(path.join("size"))?
                .parse::<Size>()
                .expect("failed to parse size"),
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

pub fn block_devices() -> io::Result<BlockDeviceIter> {
    Ok(BlockDeviceIter {
        inner: fs::read_dir("/sys/block")?,
    })
}
