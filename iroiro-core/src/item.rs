use fs::Notebook;

use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Item<'a> {
    pub name: String,
    pub path: PathBuf,
    pub is_folder: bool,
    pub notebook: &'a Notebook,
}
