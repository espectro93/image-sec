use std::error::Error;
use std::path::Path;
use clap::{App, load_yaml};

use image_sec::encrypt_image;

// + create chrome extension that can decrypt those images/or platform where we can sahre images encrypted/decrypted
fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let encrypt = matches.value_of("encrypt");

    match encrypt {
        None => (),
        Some(dir) => match encrypt_image(Path::new(dir)){
            Ok(_) => println!("Found file"),
            Err(e) => eprintln!("Error encrypting image")
        }
    }
    Ok(())
}
