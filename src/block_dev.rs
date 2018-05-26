use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fmt::Debug;
use std::fs::{self, read_to_string};
use std::io;
use std::num::ParseIntError;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub struct BlockDevice {
    /// The underlying device name such as `sda` or `mmcblk0`.
    dev_name: OsString,
    /// A human readable label or name for the device.
    label: String,
    /// The size in bytes of the device.
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
            size: read_to(dev_path.join("size"))?,
        })
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
            "{}\t{}\t{}",
            self.dev_file().display(),
            self.size(),
            self.label()
        )
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
