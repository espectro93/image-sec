mod crypt;

use std::path::Path;
use std::fs::{OpenOptions, File};
use std::error::Error;
use std::ffi::OsStr;
use std::io::{Read, Write};
use crate::crypt::{encrypt, decrypt};

pub fn encrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let (file, extension) = extract_file_with_extension(path)?;
    check_extension(extension)?;
    write_to_disk(Image::from_file(file, ActionType::Encrypt));
    Ok(())
}

pub fn decrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let (file, extension) = extract_file_with_extension(path)?;
    check_extension(extension)?;
    write_to_disk(Image::from_file(file, ActionType::Decrypt));
    Ok(())
}

pub enum ActionType {
    Encrypt,
    Decrypt,
}

struct Image {
    data: Vec<u8>
}

trait FromFile {
    fn from_file(file: File, action_type: ActionType) -> Image;
}


fn write_to_disk(image: Image) {
    let mut f = File::create("/home/steffen/CLionProjects/image-sec/resources/decrypted_image.bmp").expect("Unable to create file");
    f.write_all(&*image.data).expect("Unable to write data");
}


impl FromFile for Image {
    fn from_file(mut file: File, action_type: ActionType) -> Image {
        match action_type {
            ActionType::Encrypt => {
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        let bmp_header = &buffer[..54];
                        let mut encrypted_buffer = encrypt(buffer.as_slice());
                        let mut result_buffer = Vec::new();
                        result_buffer.extend_from_slice(&bmp_header);
                        result_buffer.append(&mut encrypted_buffer);
                        Image {
                            data: result_buffer
                        }
                    }
                    Err(_) => panic!("Error reading image")
                }
            }
            ActionType::Decrypt => {
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        let to_decrypt = &buffer[54..];
                        Image {
                            data: decrypt(to_decrypt)
                        }
                    }
                    Err(_) => panic!("Error reading image")
                }
            }
        }
    }
}

fn check_extension(extension: &str) -> Result<(), Box<dyn Error>> {
    match extension {
        "BMP" => Ok(()),
        "bmp" => Ok(()),
        _ => { panic!("Currently only bitmap images are supported"); }
    }
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
