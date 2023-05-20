use crate::utils;
use std::io::{Error, Result};
use std::path::Path;

pub fn verify_repo_root(str_path: &String) -> Result<()> {
    let expanded_path = utils::expand_tilde(str_path);
    let path = Path::new::<String>(&expanded_path);
    if !path.exists() || !path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid directory path",
        ));
    }

    return Ok(());
}
