mod crypt;

use std::path::{Path, PathBuf};
use std::fs::{OpenOptions, File};
use std::error::Error;
use std::ffi::OsStr;
use std::io::{Read, Write};
use crate::crypt::{encrypt, decrypt};
use std::env;

pub fn encrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let img_input = extract_image_input(path)?;
    check_extension(img_input.extension.as_str())?;
    write_to_disk(Image::from_image_input(img_input, ActionType::Encrypt));
    Ok(())
}

pub fn decrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let img_input = extract_image_input(path)?;
    check_extension(img_input.extension.as_str())?;
    write_to_disk(Image::from_image_input(img_input, ActionType::Decrypt));
    Ok(())
}

struct ImageInput {
    file: File,
    name: String,
    extension: String,
}

pub enum ActionType {
    Encrypt,
    Decrypt,
}

struct Image {
    name: String,
    data: Vec<u8>,
    encrypted: bool,
}

trait FromImageInput {
    fn from_image_input(img_input: ImageInput, action_type: ActionType) -> Image;
}


fn write_to_disk(image: Image) {
    let mut new_file_path: String = env::current_dir().expect("").to_str().expect("").to_string();
    new_file_path.push_str("/");
    new_file_path.push_str(image.name.as_str());
    match image.encrypted {
        true => new_file_path.push_str("_encrypted.bmp"),
        _ => new_file_path.push_str(".bmp")
    }
    let mut f = File::create(PathBuf::from(new_file_path)).expect("Unable to create file");
    f.write_all(&*image.data).expect("Unable to write data");
}


impl FromImageInput for Image {
    fn from_image_input(mut img_input: ImageInput, action_type: ActionType) -> Image {
        match action_type {
            ActionType::Encrypt => {
                let mut buffer = Vec::new();
                match img_input.file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        let bmp_header = &buffer[..54];
                        let mut encrypted_buffer = encrypt(buffer.as_slice());
                        let mut result_buffer = Vec::new();
                        result_buffer.extend_from_slice(&bmp_header);
                        result_buffer.append(&mut encrypted_buffer);
                        Image {
                            name: img_input.name,
                            data: result_buffer,
                            encrypted: true,
                        }
                    }
                    Err(_) => panic!("Error reading image")
                }
            }
            ActionType::Decrypt => {
                let mut buffer = Vec::new();
                match img_input.file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        let to_decrypt = &buffer[54..];
                        Image {
                            name: img_input.name,
                            data: decrypt(to_decrypt),
                            encrypted: false,
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

fn extract_image_input(path: &Path) -> Result<ImageInput, Box<dyn Error>> {
    let file_extension = path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Error parsing file extension");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    let name = path.file_name()
        .expect("Cannot determine file name")
        .to_str()
        .expect("Cannot convert file name to str");

    Ok(ImageInput {
        file,
        name: name.to_string(),
        extension: file_extension.to_string(),
    })
}
