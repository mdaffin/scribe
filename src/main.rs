extern crate failure;
//#[macro_use]
//extern crate failure_derive;

#[macro_use]
extern crate structopt_derive;
extern crate structopt;

use failure::Error;
use structopt::StructOpt;

use std::path::Path;
use std::io::{self, BufReader, BufWriter, Write};
use std::fs::File;

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
        Some(Command::List { all }) => println!("{:?}", all),

        None => {
            let input = &opt.input.expect("missing input file");
            let output = &opt.output.unwrap_or("out".into());
            if let Err(err) = copy(input, output) {
                println!("error copying from '{}' to '{}': {}", input, output, err);
            }
        }
    }

}

/// Copies one file to another
fn copy<A, B>(input: A, output: B) -> Result<(), Error>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    let mut buf_reader = BufReader::new(File::open(input)?);
    let mut buf_writer = BufWriter::new(File::create(output)?);

    io::copy(&mut buf_reader, &mut buf_writer)?;
    buf_writer.flush()?;

    Ok(())
}
