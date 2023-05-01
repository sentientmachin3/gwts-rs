use skim::prelude::*;
use std::io::Cursor;

pub fn fzf(repo_root: &str, worktrees: &Vec<String>) -> String {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .build()
        .unwrap();

    let input = worktrees.join("\n");
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));
    let output = Skim::run_with(&options, Some(items)).unwrap();
    let selected_item = output.selected_items.get(0).unwrap().output().to_string();
    return repo_root.to_owned() + "/" + &selected_item;
}
