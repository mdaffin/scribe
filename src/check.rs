use block_dev::BlockDevice;
use std::fs::{self, read_to_string};
use std::io;

use itertools::Itertools;

#[derive(Debug)]
pub enum Reason {
    NonRemovable,
    /// Indicates the device is mount or otherwise in use
    Mounted,
}

pub fn all(blkdev: &BlockDevice) -> Result<Option<Vec<Reason>>, io::Error> {
    let mut reasons = Vec::new();

    if is_mounted(blkdev)? {
        reasons.push(Reason::Mounted)
    }

    if !is_removable(blkdev)? {
        reasons.push(Reason::NonRemovable)
    }

    if reasons.len() == 0 {
        Ok(None)
    } else {
        Ok(Some(reasons))
    }
}

// Logic used to see if the given device is considered one the is safe to write to. There is a
// varity of crtera that will be considered for this flag. Currently it is just the removable
// flag which is not enough as some devices that are safe are marked as none removable while
// others that are no are marked. This function will deal with any corner cases that pop up.
fn is_removable(blkdev: &BlockDevice) -> Result<bool, io::Error> {
    Ok(
        if_exists!(read_to_string(blkdev.sys_path().join("removable")))?
            .map(|val| val.trim() == "1")
            .unwrap_or(false),
    )
}

fn is_mounted(blkdev: &BlockDevice) -> Result<bool, io::Error> {
    let mounts = fs::read_to_string("/proc/mounts")?;
    let mount = mounts
        .lines()
        .map(|line| line.split_whitespace().next_tuple())
        .filter_map(|line| line)
        .find(|(dev, _)| false);

    println!("{:?}", mount);

    Ok(false)
}
