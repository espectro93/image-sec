mod crypt;

use std::path::Path;
use std::fs::{OpenOptions, File, FileType};
use std::error::Error;
use std::ffi::OsStr;

struct ParsedImage {
    header: Vec<u8>,
    file_type: FileType,
    content: Vec<u8>,
}

impl ParsedImage {
    fn from_file(file: File, file_extension: &str) -> Result<(), Box<dyn Error>> {
        let metadata = file.metadata()?;
        if metadata.file_type().is_file() {
            print!("jo");
        }
        Ok(())
    }
}

pub fn encrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let (file, file_extension) = extract_file_with_extension(path)?;
    let parsed_image = ParsedImage::from_file(file, file_extension);
    Ok(())
}

fn extract_file_with_extension(path: &Path) -> Result<(File, &str), Box<dyn Error>> {
    let file_extension = path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Error parsing file extension");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    Ok((file, file_extension))
}