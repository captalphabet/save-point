pub mod tui;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SavePoints {
    pub memories: Vec<PathBuf>,
    pub memory_path: PathBuf, // oath where memories are saved: memories.json file
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

        let mut file_path: PathBuf = PathBuf::from(save_path.as_ref());
        file_path.extend(["memories.json"]);

        // Load memories from file if it exists
        match Self::load_memory(&file_path) {
            // file path is the memory file
            Ok(instance) => Ok(instance),
            Err(e) => {
                eprintln!(
                    "Failed to load from memory path, or not path, creating new instance: {e}"
                );
                let new_conf = SavePoints {
                    memory_path: file_path.to_path_buf(), // ensures points to the memories.json
                    // file in .config/save-point/
                    ..Default::default()
                };

                Ok(new_conf)
            }
        }
    }

    pub fn append_location<T: AsRef<Path>>(&mut self, path: T) {
        self.memories.push(PathBuf::from(path.as_ref()));
    }
    /// saves a path to file
    pub fn save_memory<T: AsRef<Path>>(&self, path: T) -> std::io::Result<()> {
        let save_path = path.as_ref().to_path_buf();
        // save_path.extend(["memories.json"]);

        let json = serde_json::to_string(&self.memories)?;
        let mut file = File::create(&save_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// syntactic sugar for self.save_memory(self.memory_path)
    pub fn save_to_self_path(&self) -> std::io::Result<()> {
        self.save_memory(&self.memory_path)
    }

    /// loads memorised paths from file
    /// will fail if load is called before any file actually exists
    pub fn load_memory<T: AsRef<Path>>(path: T) -> std::io::Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let json = std::fs::read_to_string(path)?;
            let save_points = SavePoints {
                memories: serde_json::from_str(&json)?,
                memory_path: path.to_owned(),
            };

            Ok(save_points)
        } else {
            Err(std::io::Error::new(
                ErrorKind::NotFound,
                "file does not exist",
            ))
        }
    }
}

#[derive(Debug)]
struct StoreError {
    source: Box<dyn Error>,
    message: &'static str,
}
#[cfg(test)]
mod save_tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_new_savepoints() {
        let save_points = SavePoints::new();
        assert!(save_points.memories.is_empty());
    }

    #[test]
    fn test_append_location() {
        let mut save_points = SavePoints::new();
        let path = PathBuf::from("example/path1");
        save_points.append_location(&path);
        assert_eq!(save_points.memories.len(), 1);
        assert_eq!(save_points.memories[0], path);
    }

    #[test]
    fn test_save_memory() -> io::Result<()> {
        let save_points = SavePoints {
            memories: vec![PathBuf::from("example/path1")],
            memory_path: PathBuf::new(),
        };
        let temp_file = "temp_save.json";
        save_points.save_memory(Path::new(temp_file))?;
        let content = fs::read_to_string(temp_file)?;
        let loaded: Vec<PathBuf> = serde_json::from_str(&content)?; // serilised content is only
                                                                    // the Vec<_> in SavePoints
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0], PathBuf::from("example/path1"));
        fs::remove_file(temp_file)?;
        Ok(())
    }

    #[test]
    fn test_load_memory() -> io::Result<()> {
        let temp_file = "temp_load.json";
        let save_points = SavePoints {
            memories: vec![PathBuf::from("example/path2")],
            memory_path: PathBuf::from(temp_file),
        };
        save_points.save_memory(Path::new(temp_file))?;
        let loaded_save_points = SavePoints::load_memory(Path::new(temp_file))?;
        assert_eq!(loaded_save_points.memories.len(), 1);
        assert_eq!(
            loaded_save_points.memories[0],
            PathBuf::from("example/path2")
        );
        fs::remove_file(temp_file)?;
        Ok(())
    }

    #[test]
    fn save_to_self_test() {
        let path = "./yolo/swag";
        let mut save_instance = SavePoints::init(path).expect("failed to create instance");

        let path_to_test_append = "yolos";
        save_instance.append_location(path_to_test_append);
        save_instance.save_to_self_path().expect("failed to save");

        drop(save_instance);
        let total_test_path = PathBuf::from(format!("{path}{path_to_test_append}"));
        let loaded_saved_points =
            SavePoints::load_memory(&total_test_path).expect("failed to load instance");

        assert_eq!(loaded_saved_points.memories[0], total_test_path);
    }
}
