extern crate failure;
#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate structopt;

use failure::Error;
use std::path::PathBuf;
use structopt::StructOpt;

mod disks;

use disks::{BlockDevice, Major};

impl Write {
    pub fn run(self) -> Result<(), Error> {
        //let block_devices = block_dev::read_devices()?.collect::<Result<Vec<_>, Error>>();
        //println!("{:?}", block_devices);
        Ok(())
    }
}

impl Backup {
    pub fn run(self) -> Result<(), Error> {
        Ok(())
    }
}

impl List {
    pub fn run(self) -> Result<(), Error> {
        for disk in BlockDevice::list()? {
            let disk = disk?;
            if disk.device_number().major != Major::ScsiDisk {
                continue;
            }
            if self.show_all | disk.is_removable() {
                println!(
                    "{}\t{}\t{}",
                    disk.path().display(),
                    disk.size(),
                    disk.device()
                        .map(|device| device.model,)
                        .unwrap_or("".into(),)
                );
            }
        }

        Ok(())
    }
}

fn main() {
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
#[structopt(name = "scribe", about = "An easy to use image writer for writing raspberry pi images to SD Cards or ISOs to USB drives.")]
enum Options {
    /// Writes an OS image to a device file
    #[structopt(name = "write")]
    Write(Write),
    /// Creates a backup of a device file
    #[structopt(name = "backup")]
    Backup(Backup),
    /// List avaiable block devices
    #[structopt(name = "list")]
    List(List),
}

#[derive(Debug, StructOpt)]
pub struct List {
    /// Show all devices including internal ones
    #[structopt(short = "a", long = "show-all")]
    show_all: bool,
}

#[derive(Debug, StructOpt)]
pub struct Backup {
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
pub struct Write {
    /// Show all devices including internal ones
    #[structopt(short = "a", long = "show-all")]
    show_all: bool,

    /// Do not ask when attempting to install to an internal drive.
    #[structopt(long = "force-internal")]
    force_internal: bool,

    /// The image to write
    #[structopt(name = "IMAGE", parse(from_os_str))]
    image: Option<PathBuf>,

    /// The device file to write the image to
    #[structopt(name = "DEVICE", parse(from_os_str))]
    device: Option<PathBuf>,
}
