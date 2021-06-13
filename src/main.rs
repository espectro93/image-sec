use std::error::Error;
use std::path::Path;

// + create chrome extension that can decrypt those images/or platform where we can sahre images encrypted/decrypted
fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let encrypt = matches.value_of("encrypt");

    match encrypt {
        None => (),
        Some(dir) => match run_daemon(Path::new(dir)){
            Ok(_) => println!("Started daemon"),
            Err(e) => eprintln!("Error starting daemon")
        }
    }
    Ok(())
}
