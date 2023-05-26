# gwts-rs
Git worktree switcher based on `fzf`, written in Rust for fun. 

## Usage
Integrates with tmux session, you can use it like this:

`gwts-rs [-s] [-w] [-r root]`

where:
- `-w` open the selected worktree in a new tmux window
- `-s` open the selected worktree in a new tmux session
- `-r root` searches for repos in the `root` directory

