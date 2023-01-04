use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    let path = "lines.txt";

    write_file(path, "Rust\nFun")?;

    let buffered = read_file(path)?;

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn read_file(path: &str) -> Result<BufReader<File>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    Ok(buffered)
}

fn write_file(path: &str, content: &str) -> Result<(), Error> {
    let mut output = File::create(path)?;
    write!(&mut output, "{}", content)?;
    Ok(())
}
