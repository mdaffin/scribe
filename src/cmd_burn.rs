use failure::Error;
use std::path::{Path, PathBuf};
use disks::{Disk, Major};
use std::io::{self, BufReader, BufWriter, Write};
use std::fs::File;
use std::iter::Iterator;

pub fn run<A, B>(input: A, output: Option<B>) -> Result<(), Error>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    let output: PathBuf = match output {
        Some(ref path) => {
            validate_device(path)?;
            path.as_ref().into()
        }
        None => get_disk_from_user()?,
    };
    println!(
        "Coping {} to {}",
        input.as_ref().display(),
        output.display()
    );
    copy(input, output)
}

fn validate_device<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let disk = match path.as_ref().file_name() {
        Some(ref p) => Disk::new(p.into())?,
        None => panic!("bad input"),
    };
    if !disk.is_removable() {
        panic!("disk is not removable");
    }
    Ok(())
}

// Ask the user for a device
fn get_disk_from_user() -> Result<PathBuf, Error> {
    let disks = Disk::list()?
        .filter(|disk| match disk {
            &Ok(ref d) => d.is_removable() && d.device_number().major == Major::ScsiDisk,
            &Err(_) => true,
        })
        .collect::<Result<Vec<_>, Error>>()?;
    for (i, disk) in disks.iter().enumerate() {
        println!(
            "{index})\t{path}\t{size}\t{model}",
            index = i,
            path = disk.path().display(),
            size = disk.size(),
            model = disk.device()
                .map(|device| device.model,)
                .unwrap_or("".into(),),
        );
    }
    print!("Select a disk: ");
    io::stdout().flush()?;
    let selected = {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().parse::<usize>()?
    };
    if selected >= disks.len() {
        // TODO handle this error better
        panic!("Not a valid disk");
    }
    Ok(PathBuf::from(disks[selected].path()))
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
