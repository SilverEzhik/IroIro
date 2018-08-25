use item::Item;

use std::fs::read_dir;
use std::path::PathBuf;

// natural ordering
extern crate natord;
use self::natord::compare_ignore_case;

impl<'a> Item<'a> {
    pub fn children(&self) -> Vec<Item> {
        if self.is_folder {
            let mut vec = vec![];

            let path = self.fs_path();
            if path.is_none() {
                return vec;
            }
            let path = path.unwrap();

            if path.is_dir() {
                let entries = read_dir(&path);
                if entries.is_err() {
                    return vec;
                }

                for entry in entries.unwrap() {
                    if entry.is_err() {
                        continue;
                    }
                    let entry_path = entry.unwrap().path();

                    let item = self.notebook.get_item(entry_path);
                    if item.is_none() {
                        continue;
                    } else {
                        vec.push(item.unwrap());
                    }
                }
            }
            vec.sort_by(|a, b| compare_ignore_case(&a.name, &b.name));
            vec
        } else {
            vec![]
        }
    }
    pub fn item_parent(&self, item: &Item) -> Option<Item> {
        let parent_path = item.path.parent();
        if parent_path.is_none() {
            return None;
        }
        let parent_path = parent_path.unwrap();

        self.notebook.get_item(parent_path)
    }

    pub fn fs_path(&self) -> Option<PathBuf> {
        self.notebook.to_fs_path(&self.path)
    }
}
