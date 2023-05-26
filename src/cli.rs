use clap::{Arg, ArgAction, Command};

pub struct CliArgs {
    pub repo_root: String,
    pub new_window: bool,
    pub new_session: bool,
}

fn get_command() -> Command {
    return Command::new("gwts")
        .author("sentientmachin3")
        .version("0.1.0")
        .about("Git worktree switcher built with fzf")
        .arg(
            Arg::new("repo-root")
                .short('r')
                .help("Repository root folder")
                .default_value("~/repos"),
        )
        .arg(
            Arg::new("new-window")
                .short('w')
                .help("Open in new tmux window")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("new-session")
                .short('s')
                .help("Open in new tmux session")
                .required(false)
                .action(ArgAction::SetTrue),
        );
}

pub fn parse_cli() -> CliArgs {
    let command = get_command();
    let matches = command.get_matches();
    let repo_root = matches
        .get_one::<String>("repo-root")
        .expect("`repo-root` is required.")
        .to_owned();
    let new_window = matches.get_one::<bool>("new-window").unwrap().to_owned();
    let new_session = matches.get_one::<bool>("new-session").unwrap().to_owned();
    if new_session && new_window {
        println!("`new_session` and `new_window` cannot be both set.");
        std::process::exit(1)
    }

    return CliArgs {
        repo_root,
        new_window,
        new_session,
    };
}
