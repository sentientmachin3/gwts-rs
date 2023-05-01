use std::fs;
use std::io::{Error, Result};
use std::path::Path;

fn list_worktrees(repo_root: &Path, repo_name: &str) -> Vec<String> {
    let mut worktrees = Vec::<String>::new();
    let full_path_str = repo_root.to_str().unwrap().to_owned() + "/" + repo_name;

    // Enter the worktrees directory and list the content
    let repo_worktrees_path = full_path_str + "/worktrees";
    let repo_wt_path = Path::new(&repo_worktrees_path);
    for wt_name_res in fs::read_dir(repo_wt_path).unwrap() {
        if wt_name_res.is_ok() {
            let wt_name = wt_name_res
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_owned();
            worktrees.push(repo_name.to_owned() + "/" + &wt_name);
        }
    }
    return worktrees;
}

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

pub fn worktree_names(repo_root: &Path) -> Result<Vec<String>> {
    let mut repo_names = Vec::new();
    for repo in fs::read_dir(repo_root)? {
        let repo = repo.unwrap();
        if repo.path().is_dir() {
            for repo_dir in fs::read_dir(repo.path())? {
                match repo_dir {
                    Ok(dir) => {
                        if dir.file_name() == "worktrees" {
                            let worktree_names =
                                list_worktrees(repo_root, repo.file_name().to_str().unwrap());
                            for name in worktree_names {
                                repo_names.push(name);
                            }
                        }
                    }
                    Err(e) => panic!("unable to read file entry {}", e.to_string()),
                }
            }
        }
    }
    return Ok(repo_names);
}

pub fn expand_tilde(tilde_path: &String) -> String {
    let home = std::env::var("HOME").unwrap();
    if tilde_path.starts_with("~") {
        return home + &tilde_path[1..];
    }
    return tilde_path.to_owned();
}
