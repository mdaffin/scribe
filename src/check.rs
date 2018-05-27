use block_dev::BlockDevice;
use std::fs::{self, read_to_string};
use std::io;
use std::fmt;
use std::path::{Path, PathBuf};

use itertools::Itertools;

#[derive(Debug)]
pub enum Reason {
    NonRemovable,
    /// Indicates the device is mount or otherwise in use
    Mounted,
}

pub fn all(blkdev: &BlockDevice) -> Result<Vec<Reason>, io::Error> {
    let mut reasons = Vec::new();

    if is_mounted(blkdev)? {
        reasons.push(Reason::Mounted)
    }

    if is_none_removable(blkdev)? {
        reasons.push(Reason::NonRemovable)
    }

    Ok(reasons)
}

// Logic used to see if the given device is considered one the is safe to write to. There is a
// varity of crtera that will be considered for this flag. Currently it is just the removable
// flag which is not enough as some devices that are safe are marked as none removable while
// others that are no are marked. This function will deal with any corner cases that pop up.
fn is_none_removable(blkdev: &BlockDevice) -> Result<bool, io::Error> {
    Ok(
        if_exists!(read_to_string(blkdev.sys_path().join("removable")))?
            .map(|val| val.trim() == "0")
            .unwrap_or(false),
    )
}

fn is_mounted(blkdev: &BlockDevice) -> Result<bool, io::Error> {
    let mounts = fs::read_to_string("/proc/mounts")?;
    Ok(mounts
        .lines()
        .map(|line| line.split_whitespace().next_tuple())
        .filter_map(|line| line) // Filter out blank lines
        .any(|(dev, _)| {
            let dev_name = blkdev.dev_file();
            let dev_name = dev_name.to_str().expect("none unicode char in device path");
            let part_name = PathBuf::from(dev);
            let part_name = part_name.to_str().expect("none unicode char in mount file");

            part_name.starts_with(dev_name)
        }))
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
