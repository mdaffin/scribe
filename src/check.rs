use block_dev::BlockDevice;
use std::io;
use std::fs;

use itertools::Itertools;

fn is_mounted(blkdev: &BlockDevice) -> Result<bool, io::Error> {
    let mounts = fs::read_to_string("/proc/mounts")?;
    let mount = mounts
        .lines()
        .map(|line| line.split_whitespace().next_tuple())
        .filter_map(|line| line)
        .find(|(dev, _)| false);

    Ok(false)
}
