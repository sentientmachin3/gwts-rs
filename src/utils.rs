use std::fs;
use std::io::{Error, Result};
use std::path::Path;

pub fn verify_repo_root(str_path: &String) -> Result<()> {
    let path = Path::new(str_path);
    if !path.exists() || !path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid directory path",
        ));
    }
    // Verify existance of the "worktrees" folder
    for file in fs::read_dir(path)? {
        match file {
            Ok(entry) => {
                if entry.file_name() == "worktrees" {
                    return Ok(());
                }
            }
            Err(e) => panic!("Unable to read file entry {}", e.to_string()),
        }
    }
    return Err(Error::new(
        std::io::ErrorKind::InvalidInput,
        "Directory does not seem to be a bare clone",
    ));
}
