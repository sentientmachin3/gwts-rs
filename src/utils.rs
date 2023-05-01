use std::fs;
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

pub fn repos_abs_paths(repo_root: &Path) -> Result<Vec<String>> {
    let mut repo_names = Vec::new();
    for repo in fs::read_dir(repo_root)? {
        let repo = repo.unwrap();
        if repo.path().is_dir() {
            for repo_dir in fs::read_dir(repo.path())? {
                match repo_dir {
                    Ok(dir) => {
                        if dir.file_name() == "worktrees" {
                            let repo_name = repo.file_name().to_str().unwrap().to_owned();
                            repo_names
                                .push(repo_root.to_str().unwrap().to_owned() + "/" + &repo_name);
                        }
                    }
                    Err(e) => panic!("unable to read file entry {}", e.to_string()),
                }
            }
        }
    }
    return Ok(repo_names);
}

pub fn verify_repo(str_path: &String) -> Result<()> {
    let expanded_path = expand_tilde(str_path);
    let path = Path::new::<String>(&expanded_path);
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

pub fn expand_tilde(tilde_path: &String) -> String {
    let home = std::env::var("HOME").unwrap();
    if tilde_path.starts_with("~") {
        return home + &tilde_path[1..];
    }
    return tilde_path.to_owned();
}

pub fn list_worktrees(repo_abs_paths: &Vec<String>) -> Vec<String> {
    let mut worktrees = Vec::<String>::new();
    for repo_path in repo_abs_paths {
        // Enter the worktrees directory and list the content
        let repo_worktrees_path = repo_path.to_owned() + "/worktrees";
        let repo_wt_path = Path::new(&repo_worktrees_path);
        for wt_name_res in fs::read_dir(repo_wt_path).unwrap() {
            if wt_name_res.is_ok() {
                let wt_name = wt_name_res
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_owned();
                worktrees.push(repo_path.to_owned() + "/" + &wt_name);
            }
        }
    }
    return worktrees;
}
