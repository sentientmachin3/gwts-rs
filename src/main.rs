use std::path::Path;
use std::process::Command;

pub mod cli;
pub mod fs_utils;
pub mod fzf;
pub mod utils;

fn main() {
    let cli_args = cli::parse_cli();
    match fs_utils::verify_repo_root(&cli_args.repo_root) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e.to_string());
            std::process::exit(1);
        }
    }

    let expanded_cli_repo_root = utils::expand_tilde(&cli_args.repo_root);
    let worktree_names = match utils::worktree_names(Path::new(&expanded_cli_repo_root)) {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err.to_string());
            std::process::exit(0);
        }
    };

    let worktree_name = fzf::fzf(&worktree_names);
    let full_path_worktree = expanded_cli_repo_root + "/" + &worktree_name;

    if cli_args.new_window {
        Command::new("tmux")
            .args(["neww", "-n", &worktree_name, "-c", &full_path_worktree])
            .spawn()
            .unwrap();
    } else {
        println!("{}", &full_path_worktree);
        Command::new("tmux")
            .args(["rename-window", &worktree_name])
            .spawn()
            .unwrap();
    }
    return;
}
