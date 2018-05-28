use itertools::Itertools;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::fmt;
use std::fs::{self, read_to_string};
use std::io;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub struct BlockDevice {
    /// The underlying device name such as `sda` or `mmcblk0`.
    dev_name: OsString,
    /// A human readable label or name for the device.
    label: String,
    /// The size in bytes of the device.
    size: Size,
    /// Flags that indicate a risky device. If any are present then the device is one we don't want
    /// to write to.
    flags: Vec<Reason>,
}

#[derive(Debug)]
pub enum Reason {
    NonRemovable,
    /// Indicates the device is mount or otherwise in use
    Mounted,
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

        let mut blkdev = BlockDevice {
            dev_name,
            label: label_parts.join(" "),
            size: read_to_string(dev_path.join("size"))?
                .trim()
                .parse()
                .expect("could not parse device size"),
            flags: Vec::new(),
        };

        blkdev.run_checks()?;
        Ok(blkdev)
    }

    pub fn flags(&self) -> &[Reason] {
        &self.flags
    }

    pub fn label<'a>(&'a self) -> &'a str {
        &self.label
    }

    pub fn dev_name(&self) -> &OsStr {
        &self.dev_name
    }

    pub fn sys_path(&self) -> PathBuf {
        PathBuf::from("/sys/block").join(self.dev_name.clone())
    }

    pub fn dev_file(&self) -> PathBuf {
        PathBuf::from("/dev").join(self.dev_name.clone())
    }

    pub fn size(&self) -> Size {
        self.size
    }

    fn run_checks(&mut self) -> Result<(), io::Error> {
        // Is removable
        if if_exists!(read_to_string(self.sys_path().join("removable")))?
            .map(|val| val.trim() == "0")
            .unwrap_or(false)
        {
            self.flags.push(Reason::NonRemovable);
        }

        // Is mounted
        if fs::read_to_string("/proc/mounts")?
            .lines()
            .map(|line| line.split_whitespace().next_tuple())
            .filter_map(|line| line) // Filter out blank lines
            .any(|(dev, _)| {
                let dev_name = self.dev_file();
                let dev_name = dev_name.to_str().expect("none unicode char in device path");
                let part_name = PathBuf::from(dev);
                let part_name = part_name.to_str().expect("none unicode char in mount file");

                part_name.starts_with(dev_name)
            }) {
            self.flags.push(Reason::Mounted);
        }

        Ok(())
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

impl fmt::Display for BlockDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:12} {:10} {:40} {}",
            self.dev_file().display(),
            self.size(),
            self.label(),
            self.flags().iter().map(|c| format!("{}", c)).join(",")
        )
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.0 * 512;
        let decimals = f.precision().unwrap_or(1);
        let string = match size {
            0...1024 => format!("{}", size),
            1024...1_048_576 => format!("{:.*}KiB", decimals, size as f64 / 1024.0),
            1_048_576...1_073_741_824 => format!("{:.*}MiB", decimals, size as f64 / 1_048_576.0),
            1_073_741_824...1_099_511_627_776 => {
                format!("{:.*}GiB", decimals, size as f64 / 1_073_741_824.0)
            }
            _ => format!("{:.*}TiB", decimals, self.0),
        };
        f.pad_integral(true, "", &string)
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Reason::NonRemovable => "non-removable",
                Reason::Mounted => "mounted",
            }
        )
    }
}
