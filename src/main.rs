extern crate failure;
extern crate termion;
//#[macro_use]
//extern crate failure_derive;

#[macro_use]
extern crate structopt_derive;
extern crate structopt;

mod disks;
mod cmd_burn;

use failure::Error;
use structopt::StructOpt;

use disks::{Disk, Major};

#[derive(StructOpt, Debug)]
#[structopt(name = "burner",
            about = "Writes images to removable media such as sd cards or flash drives")]
struct Opt {
    #[structopt(help = "Input file")]
    input: Option<String>,
    #[structopt(help = "Output file, stdout if not present")]
    output: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "list")]
    /// List available removeable drives
    List {
        #[structopt(short = "a", long = "all")]
        all: bool,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Some(Command::List { all }) => {
            if let Err(err) = cmd_list(all) {
                println!("{}", err);
            }
        }
        None => {
            if let Err(err) = cmd_burn::run(&opt.input.expect("missing input file"), opt.output) {
                println!("Burn failed: {}", err);
            }
        }
    }

}

fn cmd_list(all: bool) -> Result<(), Error> {
    for disk in Disk::list()? {
        let disk = disk?;
        if disk.device_number().major != Major::ScsiDisk {
            continue;
        }
        if all | disk.is_removable() {
            println!(
                "{}\t{}\t{}",
                disk.path().display(),
                disk.size(),
                disk.device().map(|device| device.model).unwrap_or(
                    "".into(),
                )
            );
        }
    }

    Ok(())
}
