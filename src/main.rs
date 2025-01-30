use std::{error::Error, fs::{create_dir, File}, io::Write, path::{Path, PathBuf}, str::FromStr};
use serde::{Serialize, Deserialize};

fn main() {

    let current_dir = get_current_path().expect("failed to get current path");
    println!("{:?}",current_dir);
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
        Ok(path_string) => PathBuf::from_str(&path_string)?,
        Err(_e) => PathBuf::from_str("~/.config/save-point")?, // default save directory
        // if SAVEPOINT_SAV_DIR env variable not set


    };

    // Test path exists or not
    //
    if !store_path.is_dir() { // either a directory or something failed not a dir, means doesnt
        // exist
        create_dir(&store_path)?; // Not a dir? create it


    }


    Ok(store_path)
}



/// Something to store locations to that is serialisable
#[derive(Debug,Deserialize,Serialize)]
struct SavePoints {
    memories: Vec<PathBuf>,
}

impl SavePoints {
    pub fn new() -> Self {
        Self {
            memories: Vec::new()
        }



    }

    pub fn append_location(&mut self, path: &Path) {
        self.memories.push(PathBuf::from(path));


    }

    pub fn save_memory(&self, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        todo!("Need to finish saving the SavePoint object to file");
    

        
    }
}

#[derive(Debug)]
struct StoreError {
    source: Box<dyn Error>,
    message: &'static str


}


