use itertools::Itertools;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, read_to_string};
use std::io;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[cfg(not(test))]
static PROC_MOUNTS: &'static str = "/proc/mounts";
#[cfg(test)]
static PROC_MOUNTS: &'static str = "src/tests/mounts";

#[derive(Debug, PartialEq)]
pub struct BlockDevice {
    /// The sysfs block device path
    sys_path: PathBuf,
    /// A human readable label or name for the device.
    label: String,
    /// The size in bytes of the device.
    size: Size,
    /// The detected general type of the device.
    device_type: DeviceType,
    /// Flags that indicate a risky device. If any are present then the device is one we don't want
    /// to write to.
    flags: Vec<Reason>,
}

/// The general type of a block device. FlashDrives and SDMMC are considered safe to write to
/// while other devices are not.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DeviceType {
    /// USB flash drives, typically devices that you will want to write OS and Live USB images to.
    /// Note that this can include some SDMMC adaptors that present themselves as SCSI devices.
    FlashDrive,
    /// SD/MMC Cards and card readers. Typically what you would write raspberry pi images or images
    /// for other embedded devices. Note that some adaptors will look more like USB flash drives.
    SDMMC,
    /// Any internal drive. There is a case for using being able to force a write to these devices
    /// but should be hidden by default.
    InternalDrive,
    /// Any external drive that is not a good candidate for a writing OS images to, such as USB
    /// HDDs. There is a case for using being able to force a write to these devices but should be
    /// hidden by default.
    ///
    /// *Note* I have not yet found a way to tell these appart from InternalDrives so this is
    /// currently unused and may be dropped in the future.
    ExternalDrive,
    /// A cd-rom drive. These are block devices but should never be considered for possible
    /// location to write to.
    CDROM,
    /// Loopback devices. These are virtual devices and should never be considered for possible
    /// locations to write to.
    LoopBack,
}

/// Additional reasons why a device might not be considered safe to write an OS image to, such as
/// it the device is already mounted or too small for a given image.
#[derive(Debug, PartialEq)]
pub enum Reason {
    /// Indicates the device is mount or otherwise in use
    Mounted,
    /// A device that has a size of 0. This is normally SD Card readers that do not have an SD Card
    /// inserted.
    ZeroSize,
    /// A device that is marked as read only such as SD Cards with the write lock switch set.
    ReadOnly,
    /// A large device, one that is so big that writing an OS image to it is a waste and likely not
    /// wanted. These are more likely to be removable USB HDDs for backups or extra storage and so
    /// should not be listed by default. This includes devices that are >36GB in size (ie ~32GB
    /// devices are ok with some buffer for variation in device size).
    Large,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub fn new(sys_path: PathBuf) -> Result<BlockDevice, io::Error> {
        let mut label_parts = Vec::with_capacity(2);

        let vendor = if_exists!(read_to_string(sys_path.join("device/vendor")))?;
        let model = if_exists!(read_to_string(sys_path.join("device/model")))?;

        if let Some(vendor) = vendor {
            label_parts.push(vendor.trim().to_string())
        }

        if let Some(model) = model {
            label_parts.push(model.trim().to_string())
        }

        let size = read_to_string(sys_path.join("size"))
            .expect("failed to read size")
            .trim()
            .parse()
            .expect("could not parse device size");

        let device_type = BlockDevice::workout_type(&sys_path)?;

        Ok(BlockDevice {
            sys_path,
            label: label_parts.join(" "),
            size,
            device_type,
            flags: Vec::new(),
        })
    }

    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    pub fn flags(&self) -> &[Reason] {
        &self.flags
    }

    pub fn label<'a>(&'a self) -> &'a str {
        &self.label
    }

    pub fn dev_name(&self) -> &OsStr {
        &self.sys_path
            .file_name()
            .expect("missing file name on device path")
    }

    pub fn sys_path(&self) -> &Path {
        self.sys_path.as_path()
    }

    pub fn dev_file(&self) -> PathBuf {
        PathBuf::from("/dev").join(self.dev_name())
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn workout_type(blkdev_path: impl AsRef<Path>) -> Result<DeviceType, io::Error> {
        let dev_name = blkdev_path
            .as_ref()
            .file_name()
            .expect("missing file name on device")
            .to_str()
            .expect("none unicode char in device name");

        if dev_name.starts_with("mmcblk") {
            Ok(DeviceType::SDMMC)
        } else if dev_name.starts_with("sd") {
            if read_to_string(blkdev_path.as_ref().join("removable"))
                .map(|val| val.trim() == "0")
                .expect("error reading the 'removable' file")
            {
                Ok(DeviceType::InternalDrive)
            } else {
                Ok(DeviceType::FlashDrive)
            }
        } else if dev_name.starts_with("sr") {
            Ok(DeviceType::CDROM)
        } else if dev_name.starts_with("loop") {
            Ok(DeviceType::LoopBack)
        } else {
            Ok(DeviceType::InternalDrive)
        }
    }
}

impl DeviceType {
    /// Returns true if the device type is considered a safe type to write OS images to.
    pub fn is_safe(&self) -> bool {
        match self {
            DeviceType::FlashDrive => true,
            DeviceType::SDMMC => true,
            DeviceType::InternalDrive => false,
            DeviceType::ExternalDrive => false,
            DeviceType::CDROM => false,
            DeviceType::LoopBack => false,
        }
    }

    /// Returns true if the device type is to always be excluded from listings.
    pub fn is_excluded(&self) -> bool {
        match self {
            DeviceType::FlashDrive => false,
            DeviceType::SDMMC => false,
            DeviceType::InternalDrive => false,
            DeviceType::ExternalDrive => false,
            DeviceType::CDROM => true,
            DeviceType::LoopBack => true,
        }
    }
}

fn run_checks(blkdev: &mut BlockDevice) -> Result<(), io::Error> {
    // Is removable

    // Is mounted
    if read_to_string(PROC_MOUNTS)?
        .lines()
        .filter_map(|line| line.split_whitespace().next_tuple())
        .any(|(dev, _)| {
            let dev_name = blkdev.dev_file();
            let dev_name = dev_name.to_str().expect("none unicode char in device path");
            let part_name = PathBuf::from(dev);
            let part_name = part_name.to_str().expect("none unicode char in mount file");

            part_name.starts_with(dev_name)
        }) {
        blkdev.flags.push(Reason::Mounted);
    }

    Ok(())
}

impl Iterator for BlockDeviceIter {
    type Item = Result<BlockDevice, io::Error>;

    fn next(&mut self) -> Option<Result<BlockDevice, io::Error>> {
        loop {
            return match self.inner.next() {
                Some(Ok(dir)) => {
                    let mut blkdev = BlockDevice::new(dir.path());
                    if let Ok(ref mut blkdev) = blkdev {
                        if let Err(err) = run_checks(blkdev) {
                            return Some(Err(err));
                        }
                    }
                    Some(blkdev)
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
        f.pad_integral(
            true,
            "",
            &format!(
                "{:10} {:10} {:25} {:10} {}",
                self.dev_file().display(),
                self.size(),
                self.label(),
                self.device_type,
                self.flags().iter().map(|c| format!("{}", c)).join(",")
            ),
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

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(
            true,
            "",
            &format!(
                "{}",
                match self {
                    DeviceType::FlashDrive => "Flash Drive",
                    DeviceType::SDMMC => "SD/MMC Card",
                    DeviceType::InternalDrive => "Internal Drive",
                    DeviceType::ExternalDrive => "External Drive",
                    DeviceType::CDROM => "CD-ROM",
                    DeviceType::LoopBack => "LoopBack",
                }
            ),
        )
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad_integral(
            true,
            "",
            &format!(
                "{}",
                match self {
                    Reason::Mounted => "mounted",
                    Reason::ZeroSize => "zero-size",
                    Reason::ReadOnly => "read-only",
                    Reason::Large => "large",
                }
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_dir;

    struct DeviceTestCase {
        device_type: DeviceType,
    }

    fn sysfs() -> PathBuf {
        PathBuf::from(file!()).parent().unwrap().join("tests/sysfs")
    }

    #[test]
    fn device_checks() {
        for res in read_dir(sysfs()).unwrap() {
            let dir = res.unwrap();
            let blkdev = BlockDevice::new(dir.path()).unwrap();
            let test_case = load_device_test(dir.path());
            assert_eq!(test_case.device_type, blkdev.device_type);
        }
    }

    fn load_device_test(src: impl AsRef<Path>) -> DeviceTestCase {
        DeviceTestCase {
            device_type: match read_to_string(src.as_ref().join("scribe_type"))
                .unwrap()
                .trim()
            {
                "FlashDrive" => DeviceType::FlashDrive,
                "SDCard" => DeviceType::SDMMC,
                "InternalDrive" => DeviceType::InternalDrive,
                "ExternalDrive" => DeviceType::ExternalDrive,
                "CDROM" => DeviceType::CDROM,
                "LoopBack" => DeviceType::LoopBack,
                v => panic!("not a valid device type: {}", v),
            },
        }
    }
}
