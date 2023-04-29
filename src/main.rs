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
}
