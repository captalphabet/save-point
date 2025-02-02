use dirs::home_dir;
use save_point::SavePoints;
use std::error::Error;
use std::fs::{create_dir, create_dir_all};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;
use save_point::tui::run_tui;

fn main() -> Result<(), Box<dyn Error>> {
    let store_path = setup_location_store()?;

    let mut store = SavePoints::init(&store_path)?;





    _ = run_tui(&mut store);

    

    Ok(())
}

// Save current path to a location
// 1. Get current path "pwd"
//


/// Initialises the save point directory
fn setup_location_store() -> Result<PathBuf, Box<dyn Error>> {
    let mut default_home_config = home_dir().expect("failed to Retrieve OS HOME");
    default_home_config.extend([".config","save-point"].iter());

    let store_path = match std::env::var("SAVEPOINT_SAVE_DIR") {
        // Define store directory from ENV varible or from default
        Ok(path_string) => PathBuf::from_str(&path_string)?,
        Err(_e) => default_home_config, // default save directory
                                 // if SAVEPOINT_SAV_DIR env variable not set
    };

    dbg!(&store_path);

    // Test path exists or not
    //
    if !store_path.is_dir() {
        // either a directory or something failed not a dir, means doesnt
        // exist
        match create_dir(&store_path) {
            // Not a dir? create it
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::NotFound => {
                dbg!("Parent Directory Missing");
            }

            _ => (),
        };
    }

    Ok(store_path)
}

/// Confirming create_dir_all does not overrite parent directories
fn _test_create_dir_all() {
    create_dir_all("./meme/cap").expect("failed to create nested dirs");
}

fn _test_home_dir() {
    let home_dir = home_dir().expect("failed to get home directory");
    let dir_content = std::fs::read_dir(home_dir).expect("failed to read dir");

    for entry_res in dir_content {
        let entry = entry_res.expect("failed to unwrap DirEntryRes");
        println!("{:?}", entry);
    }
}
