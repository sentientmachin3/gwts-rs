use std::path::Path;

pub mod cli;
pub mod utils;

fn main() {
    let cli_args = cli::parse_cli();
    match utils::verify_repo_root(&cli_args.repo_root) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e.to_string());
            std::process::exit(1);
        }
    }

    let expanded_cli_repo_root = utils::expand_tilde(&cli_args.repo_root);
    let worktree_repos_abs = utils::repos_abs_paths(Path::new(&expanded_cli_repo_root)).unwrap();
    let worktrees = utils::list_worktrees(&worktree_repos_abs);
}
