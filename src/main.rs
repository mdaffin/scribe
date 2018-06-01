#[macro_use]
extern crate failure;
#[macro_use]
extern crate human_panic;
extern crate itertools;
#[macro_use]
extern crate structopt;
//#[macro_use]
extern crate log;
extern crate simplelog;
extern crate termion;

use failure::Error;
use simplelog::{Config, LevelFilter, TermLogger};
use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
mod util;
mod block_dev;
mod menus;

use block_dev::block_devices;

/// Returns true is the device should be included in listings
fn include_dev(blkdev: &block_dev::BlockDevice, show_all: bool) -> bool {
    // Filter out all excludable devices (like loopback and cd-roms) as we
    // never want to write to them.
    !blkdev.device_type().is_excluded()
        && (show_all || blkdev.device_type().is_safe() && blkdev.flags().len() == 0)
}

impl WriteCmd {
    pub fn run(self) -> Result<(), Error> {
        check_tty()?;

        let devices = block_dev::block_devices()?
            .filter(|dev| {
                dev.as_ref()
                    .and_then(|dev| Ok(include_dev(dev, self.show_all)))
                    .unwrap_or(true)
            })
            .collect::<Result<Vec<_>, io::Error>>()?;
        let selected = match menus::select_from(&devices) {
            None => return Ok(()),
            Some(dev) => dev,
        };

        println!(
            "Writing '{}' to device '{}'. This will take a while",
            self.image.display(),
            selected.dev_file().display()
        );

        let mut image_file = File::open(self.image)?;
        let mut device_file = OpenOptions::new()
            .write(true)
            .truncate(false)
            .open(selected.dev_file())?;

        // TODO preform checks:
        // - check length of image and device

        io::copy(&mut image_file, &mut device_file)?;

        println!("Flushing data. This will take a while");

        device_file.sync_all()?;

        println!(
            "Finished. {} is now safe to remove.",
            selected.dev_file().display()
        );

        Ok(())
    }
}

impl BackupCmd {
    pub fn run(self) -> Result<(), Error> {
        check_tty()?;
        Ok(())
    }
}

impl ListCmd {
    pub fn run(self) -> Result<(), Error> {
        for disk in block_devices()? {
            let disk = disk?;
            if include_dev(&disk, self.show_all) {
                println!("{}", disk)
            }
        }
        Ok(())
    }
}

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    setup_panic!();
    if let Err(err) = match Options::from_args() {
        Options::Write(c) => c.run(),
        Options::Backup(c) => c.run(),
        Options::List(c) => c.run(),
    } {
        println!("{}", err)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "scribe",
            about = "An easy to use image writer for writing raspberry pi images to SD Cards or ISOs to USB drives.")]
enum Options {
    /// Writes an OS image to a device file
    #[structopt(name = "write")]
    Write(WriteCmd),
    /// Creates a backup of a device file
    #[structopt(name = "backup")]
    Backup(BackupCmd),
    /// List avaiable block devices
    #[structopt(name = "list")]
    List(ListCmd),
}

#[derive(Debug, StructOpt)]
pub struct ListCmd {
    /// Show all devices including internal ones
    #[structopt(short = "a", long = "show-all")]
    show_all: bool,
    /// List the reasons why a device is considered not safe (implies --show-all)
    #[structopt(short = "r", long = "reasons")]
    reasons: bool,
}

#[derive(Debug, StructOpt)]
pub struct BackupCmd {
    /// Show all devices including internal ones
    #[structopt(short = "a", long = "show-all")]
    show_all: bool,

    /// The device file to read the image from
    #[structopt(name = "DEVICE", parse(from_os_str))]
    device: Option<PathBuf>,

    /// The name of the image to create
    #[structopt(name = "IMAGE", parse(from_os_str))]
    image: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct WriteCmd {
    /// Show all devices including internal ones
    #[structopt(short = "a", long = "show-all")]
    show_all: bool,

    /// Do not ask when attempting to install to an internal drive.
    #[structopt(long = "force-internal")]
    force_internal: bool,

    /// The image to write
    #[structopt(name = "IMAGE", parse(from_os_str))]
    image: PathBuf,

    /// The device file to write the image to
    #[structopt(name = "DEVICE", parse(from_os_str))]
    device: Option<PathBuf>,
}

/// Returns an error if there is no tty attached to both stdin and stderr.
fn check_tty() -> Result<(), Error> {
    use std::io::{stdin, stdout};
    let stdout = stdout();
    let stdin = stdin();
    if !termion::is_tty(&stdout) || !termion::is_tty(&stdin) {
        bail!("Scribe requires a TTY to function and there was none found.");
    }
    Ok(())
}
