#[macro_use]
mod util;

use std::path::PathBuf;
use std::ffi::OsString;
use std::io;
use std::fs;
use failure;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

use std::fs::read_to_string;
use self::util::{read_to, IntBool};

#[derive(Debug)]
pub struct BlockDevice {
    dev_name: OsString,
    label: String,
    external: bool,
    size: Size,
}

#[derive(Debug, Copy, Clone)]
pub struct Size(pub u64);

pub struct BlockDeviceIter {
    inner: fs::ReadDir,
}

pub fn block_devices() -> io::Result<BlockDeviceIter> {
    Ok(BlockDeviceIter {
        inner: fs::read_dir("/sys/block")?,
    })
}

impl BlockDevice {
    pub fn new(dev_name: OsString) -> Result<BlockDevice, io::Error> {
        let dev_path = PathBuf::from("/sys/block").join(dev_name.clone());
        let mut label_parts = Vec::new();

        let vendor = if_exists!(read_to_string(dev_path.join("device/vendor")))?;
        let model = if_exists!(read_to_string(dev_path.join("device/model")))?;

        if let Some(vendor) = vendor {
            label_parts.push(vendor.trim().to_string())
        }

        if let Some(model) = model {
            label_parts.push(model.trim().to_string())
        }

        Ok(BlockDevice {
            dev_name,
            label: label_parts.join(" "),
            external: true,
            size: read_to(dev_path.join("size"))?,
        })
    }

    pub fn external(&self) -> bool {
        self.external
    }

    pub fn dev_file(&self) -> PathBuf {
        PathBuf::from("/dev").join(self.dev_name.clone())
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl Iterator for BlockDeviceIter {
    type Item = Result<BlockDevice, io::Error>;

    fn next(&mut self) -> Option<Result<BlockDevice, io::Error>> {
        loop {
            return match self.inner.next() {
                Some(Ok(dir)) => {
                    // Loopback devices do not have a devices directory, but all other physical
                    // devices that I could find do so this seems to be a way to filter them out.
                    if !dir.path().join("device").exists() {
                        continue;
                    };
                    Some(BlockDevice::new(
                        dir.path()
                            .file_name()
                            .expect(&format!("missing file_name for '{}'", dir.path().display()))
                            .into(),
                    ))
                }
                Some(Err(err)) => Some(Err(err.into())),
                None => None,
            };
        }
    }
}

impl FromStr for Size {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Size(s.parse::<u64>()?))
    }
}

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
