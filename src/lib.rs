mod crypt;

use std::path::Path;
use std::fs::{OpenOptions, File, FileType};
use std::error::Error;
use std::ffi::OsStr;
use std::io::{Read, Write};
use crate::crypt::{encrypt, decrypt};
use std::fs;
use image::{GenericImageView, ImageBuffer, Rgba, RgbImage, DynamicImage, Rgb, EncodableLayout, ImageFormat};


fn from_file(mut file: File) -> Vec<u8> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);
    let bmp_header = &buffer[..54];
    let mut encrypted_buffer = encrypt(buffer.as_slice());
    let mut result_buffer = Vec::new();
    result_buffer.extend_from_slice(&bmp_header);
    result_buffer.append(&mut encrypted_buffer);
    result_buffer
}

fn write_to_disk(data: Vec<u8>) {
    let mut f = File::create("/home/steffen/CLionProjects/image-sec/resources/decrypted_image.bmp").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}

pub fn encrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let (file, extension) = extract_file_with_extension(path)?;
    //TODO: CHECK EXTENSION AND PARSE TO BMP IF NECESSARY
    write_to_disk(from_file(file));
    Ok(())
}

fn decrypt_from_file(mut file: File) -> Vec<u8>{
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);
    let to_decrypt = &buffer[54..];
    decrypt(to_decrypt)
}

pub fn decrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let (file, extension) = extract_file_with_extension(path)?;
    write_to_disk(decrypt_from_file(file));
    Ok(())
}

pub fn open_image_and_decrypt(path: &Path) -> Result<(), Box<dyn Error>> {
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