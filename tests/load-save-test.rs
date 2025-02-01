#[cfg(test)]
mod tests {
    
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::{PathBuf, Path};
    use save_point::SavePoints;

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
        };
        let temp_file = "temp_save.json";
        save_points.save_memory(Path::new(temp_file))?;
        let content = fs::read_to_string(temp_file)?;
        let loaded: SavePoints = serde_json::from_str(&content)?;
        assert_eq!(loaded.memories.len(), 1);
        assert_eq!(loaded.memories[0], PathBuf::from("example/path1"));
        fs::remove_file(temp_file)?;
        Ok(())
    }

    #[test]
    fn test_load_memory() -> io::Result<()> {
        let temp_file = "temp_load.json";
        let save_points = SavePoints {
            memories: vec![PathBuf::from("example/path2")],
        };
        save_points.save_memory(Path::new(temp_file))?;
        let loaded_save_points = SavePoints::load_memory(Path::new(temp_file))?;
        assert_eq!(loaded_save_points.memories.len(), 1);
        assert_eq!(loaded_save_points.memories[0], PathBuf::from("example/path2"));
        fs::remove_file(temp_file)?;
        Ok(())
    }
}
