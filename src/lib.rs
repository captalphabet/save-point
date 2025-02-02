
pub mod tui;
use std::{error::Error, fs::{File}, io::Write, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize,Serialize,Default)]
pub struct SavePoints {
     memories: Vec<PathBuf>,
}

pub fn get_current_path() -> std::io::Result<PathBuf> {
    std::env::current_dir()
}

impl SavePoints {
    pub fn new() -> Self {

        Self::default()



    }
    /// start a SavePoints instance from a path to a directory to create file into
    pub fn init<T: AsRef<Path>>(save_path: T) -> std::io::Result<Self> {
        // save_path is the directory path

        let mut file_path = save_path.as_ref().to_path_buf();
        file_path.extend(["memories.json"]);
        
        // Load memories from file if it exists
        match Self::load_memory(&file_path) { // file path is the memory file
            Ok(instance) => Ok(instance),
            Err(e) => {
                eprintln!("Failed to load from memory path, or not path, creating new instance");
                Ok(Self::new())
            }
        }
        


    }

    pub fn append_location(&mut self, path: &Path) {
        self.memories.push(PathBuf::from(path));


    }
    /// saves a path to file
    pub fn save_memory(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())

    

        
    }
    /// loads memorised paths from file
    pub fn load_memory(path: &Path) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let save_points = serde_json::from_str(&json)?;
        Ok(save_points)
    }
}

#[derive(Debug)]
struct StoreError {
    source: Box<dyn Error>,
    message: &'static str


}
