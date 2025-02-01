use std::fs::{create_dir, create_dir_all};
use std::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;

fn main() -> Result<(),Box<dyn Error>> {

    let store_path = setup_location_store()?;



    Ok(())

}




// Save current path to a location
// 1. Get current path "pwd"
//

fn get_current_path() -> std::io::Result<PathBuf> {

    std::env::current_dir()


}

/// Initialises the save point directory
fn setup_location_store() -> Result<PathBuf,Box<dyn Error>> {
   let store_path =  match std::env::var("SAVEPOINT_SAVE_DIR") {
        // Define store directory from ENV varible or from default
        Ok(path_string) => PathBuf::from_str(&path_string)?,
        Err(_e) => PathBuf::from_str("~/.config/save-point")?, // default save directory
        // if SAVEPOINT_SAV_DIR env variable not set


    };

    // Test path exists or not
    //
    if !store_path.is_dir() { // either a directory or something failed not a dir, means doesnt
        // exist
        match create_dir_all(&store_path) { // Not a dir? create it
            
            Ok(_) => {},
            Err(e) if e.kind() == ErrorKind::NotFound => {
                dbg!("Uhoh no parent");


            }

            _ => (),



        };

    }


    Ok(store_path)
}

/// Confirming create_dir_all does not overrite parent directories
fn _test_create_dir_all(){
    create_dir_all("./meme/cap").expect("failed to create nested dirs");
}






