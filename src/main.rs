use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    let plain_file_path = "lines.txt";
    let enc_file_path = "lines_enc.txt";

    encrypt(plain_file_path, enc_file_path)?;

    Ok(())
}

fn encrypt(plain_file_path: &str, enc_file_path: &str) -> Result<(), Error> {
    let buffered = read_file(plain_file_path)?;

    let mut encrypted: String = "".to_string();
    for line in buffered.lines() {
        encrypted.push_str(&rot13_encrypt_content(line?));
        encrypted.push_str("\n");
    }

    write_file(enc_file_path, &encrypted)?;

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

fn rot13_encrypt_content(content: String) -> String {
    rot13::rot13(&content)
}
