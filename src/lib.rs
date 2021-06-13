mod crypt;

use std::path::Path;
use std::fs::{OpenOptions, File, FileType};
use std::error::Error;

struct ParsedImage {
    header: Vec<u8>,
    file_type: FileType,
    content: Vec<u8>,
}

impl ParsedImage {
    fn from_file(file: File) -> Result<Self, Box<dyn Error>> {
        let metadata = file.metadata()?;
        match metadata.file_type(){
            FileType("png")=> {
                //keep first 8 bytes and encrypt the rest, rebuild image
                panic!()
            }
            _ => {panic!()}
        }
    }
}

pub fn encrypt_image(path: Path) -> Result<ParsedImage, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    let parsed_image = ParsedImage::from_file(file);

}