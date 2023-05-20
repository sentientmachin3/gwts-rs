use std::fs;
use std::io::{Error, Result};
use std::path::Path;

fn list_worktrees(path: &Path) -> Vec<String> {
    let mut full_path_worktrees = Vec::<String>::new();
    for wt_name_res in fs::read_dir(path).unwrap() {
        if wt_name_res.is_ok() {
            let wt_name = wt_name_res
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_owned();
            let full_worktree_path = path.to_str().unwrap().to_owned();
            full_path_worktrees.push(full_worktree_path + "/" + &wt_name);
        }
    }
    return full_path_worktrees;
}

pub fn get_worktree_names(repo_root: &Path) -> Result<Vec<String>> {
    let mut worktree_full_paths = Vec::<String>::new();
    for repo in fs::read_dir(repo_root)? {
        let repo = repo.unwrap();
        if repo.path().is_dir() {
            let worktree_dir_path_str = repo.path().to_str().unwrap().to_owned() + "/worktrees";
            let worktree_dir_path = Path::new(&worktree_dir_path_str);
            if worktree_dir_path.exists() {
                let mut worktree_names = list_worktrees(worktree_dir_path);
                worktree_full_paths.append(&mut worktree_names);
            }
        }
    }
    if worktree_full_paths.len() == 0 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "No repos in the base folder",
        ));
    }
    return Ok(worktree_full_paths);
}
