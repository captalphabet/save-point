
pub mod tui;
use std::{error::Error, fs::{File}, io::Write, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize,Serialize,Default)]
pub struct SavePoints {
    pub memories: Vec<PathBuf>,
}

impl SavePoints {
    pub fn new() -> Self {

        Self::default()



    }
    /// start a SavePoints instance from a prepopulated path
    pub fn init<T: AsRef<Path>>(save_path: T) {


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
