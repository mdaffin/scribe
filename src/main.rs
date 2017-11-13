use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

fn main() {
    let in_file = File::open("src/main.rs").unwrap();
    let mut buf_reader = BufReader::new(in_file);

    let out_file = File::create("test.rs").unwrap();
    let mut buf_writer = BufWriter::new(out_file);

    io::copy(&mut buf_reader, &mut buf_writer).unwrap();
}
