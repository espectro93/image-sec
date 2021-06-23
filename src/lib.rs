mod crypt;

use std::path::Path;
use std::fs::{OpenOptions, File, FileType};
use std::error::Error;
use std::ffi::OsStr;
use std::io::{Read, Write};
use crate::crypt::{encrypt, decrypt};
use std::fs;
use image::{GenericImageView, ImageBuffer, Rgba, RgbImage, DynamicImage, Rgb, EncodableLayout, ImageFormat};

pub fn encrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    open_img_and_encrypt(path);
    Ok(())
}

fn open_img_and_encrypt(path: &Path) -> Result<(), Box<dyn Error>> {
    match image::open(path) {
        Ok(img) => {
            let mut encrypted_content = encrypt(&img.as_bytes());
            let image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::from_raw(img.width(), img.height(), encrypted_content).expect("Cannot create image buffer!");
            image_buffer.save(create_new_filename_from(path, "/encrypted_"));
        }
        Err(e) => panic!("Problem opening the image: {:?}", e),
    }
    Ok(())
}

fn create_new_filename_from(path: &Path, prefix: &str) -> String {
    path.parent().unwrap().to_str().unwrap().to_owned() + &*prefix.to_owned() + &*path.file_name().unwrap().to_str().unwrap().to_owned()
}

pub fn decrypt_image(path: &Path) -> Result<(), Box<dyn Error>> {
    open_image_and_decrypt(path);
    Ok(())
}

pub fn open_image_and_decrypt(path: &Path) -> Result<(), Box<dyn Error>> {
    //TODO: Mit Bitmap muss es eigtl. gehen, sonst nochmal Artikel checken
    //Vllt. doch ohne Image crate bzw. eher für Conversion, wir nehmen die ersten 54 Bytes des Original Images
    //encrypten gesamtes image, erstelln buffer mit 54 bytes des original images und appenden den encrypteten kram
    //zum decrypten entfernen wir die ersten 54 bytes
    match image::open(path) {
        Ok(img) => {
            let mut decrypted_content = decrypt(img.as_bytes());
            let image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::from_raw(img.width(), img.height(), decrypted_content).expect("Cannot create image buffer!");
            image_buffer.save(create_new_filename_from(path, "/"));
        }
        Err(e) => panic!("Problem opening the image: {:?}", e),
    }
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