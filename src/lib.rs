mod crypt;

use std::path::Path;
use std::fs::{OpenOptions, File, FileType};
use std::error::Error;
use std::ffi::OsStr;
use std::io::{Read, Write};
use crate::crypt::{encrypt, decrypt};
use std::fs;
use image::{GenericImageView, ImageBuffer, Rgba, RgbImage, DynamicImage, Rgb, EncodableLayout, ImageFormat};

enum ImageExtension {
    PNG,
    BMP,
    JPG,
    JPEG,
}

struct EncryptedImage {
    data: Vec<u8>
}

struct DecryptedImage {
    data: Vec<u8>
}

pub trait FromFile<T> {
    fn from_file(mut file: File) -> T;
}

pub trait WriteToDisk<T> {
    fn write_to_disk(mut t: T) -> Vec<u8>;
}

impl FromFile<EncryptedImage> for EncryptedImage {
    fn from_file(mut file: File) -> EncryptedImage {
        unimplemented!()
    }
}

impl FromFile<DecryptedImage> for DecryptedImage {
    fn from_file(mut file: File) -> DecryptedImage {
        unimplemented!()
    }
}

impl WriteToDisk<EncryptedImage> for EncryptedImage {
    fn write_to_disk(mut t: EncryptedImage) -> Vec<u8> {
        unimplemented!()
    }
}

impl WriteToDisk<DecryptedImage> for DecryptedImage {
    fn write_to_disk(mut t: DecryptedImage) -> Vec<u8> {
        unimplemented!()
    }
}

//TODO: DEFINE FROM_FILE AS TRAIT
impl EncryptedImage {
    pub fn from_file(mut file: File) -> Self {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer);
        let bmp_header = &buffer[..54];
        let mut encrypted_buffer = encrypt(buffer.as_slice());
        let mut result_buffer = Vec::new();
        result_buffer.extend_from_slice(&bmp_header);
        result_buffer.append(&mut encrypted_buffer);
        return EncryptedImage {
            data: result_buffer
        };
    }
}

impl DecryptedImage {
    pub fn from_file(mut file: File) -> Self {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer);
        let to_decrypt = &buffer[54..];
        return DecryptedImage {
            data: decrypt(to_decrypt)
        };
    }
}

pub fn encrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let (file, extension) = extract_file_with_extension(path)?;
    //TODO: CHECK EXTENSION AND PARSE TO BMP IF NECESSARY
    write_to_disk(EncryptedImage::from_file(file));
    Ok(())
}

pub fn decrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    let (file, extension) = extract_file_with_extension(path)?;
    write_to_disk(DecryptedImage::from_file(file));
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

//TODO: DO THIS OVER A TRAIT THAT IS PASSED IN AND DELIVERS DATA
fn write_to_disk(data: Vec<u8>) {
    //TODO: DETERMINE CURRENT DIRECTORY AND WRITE TO THAT
    let mut f = File::create("/home/steffen/CLionProjects/image-sec/resources/decrypted_image.bmp").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
