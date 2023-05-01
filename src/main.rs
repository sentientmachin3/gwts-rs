use std::path::Path;

pub mod cli;
pub mod utils;
pub mod fzf;

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
    let worktree_names = utils::worktree_names(Path::new(&expanded_cli_repo_root)).unwrap();
    let selected_wt = fzf::fzf(&expanded_cli_repo_root, &worktree_names);
}
