use std::io::{Error, Result};
use std::path::Path;

pub fn verify_repo_root(str_path: &String) -> Result<()> {
    let expanded_path = expand_tilde(str_path);
    let path = Path::new::<String>(&expanded_path);
    if !path.exists() || !path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid directory path",
        ));
    }

    return Ok(());
}

pub fn expand_tilde(tilde_path: &String) -> String {
    let home = std::env::var("HOME").unwrap();
    if tilde_path.starts_with("~") {
        return home + &tilde_path[1..];
    }
    return tilde_path.to_owned();
}
