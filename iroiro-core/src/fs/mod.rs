use item::Item;

use std::path::{Path, PathBuf};

mod item;

#[derive(PartialEq, Debug)]
pub struct Notebook {
    notebook_path: PathBuf,
    notes_path: PathBuf,
}

impl Notebook {
    pub fn get_root(&self) -> Option<Item> {
        self.get_item("")
    }

    pub fn to_fs_path<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        let path = path.as_ref();

        let mut abs_path = PathBuf::new();
        if path.is_absolute() {
            abs_path.push(path);
        } else {
            abs_path.push(&self.notes_path);
            abs_path.push(path);
        }

        // don't go above the Notes folder
        abs_path = abs_path.canonicalize().unwrap_or(PathBuf::new());
        if !abs_path.starts_with(self.notes_path.as_path()) || abs_path.file_stem().is_none() {
            None
        } else {
            Some(abs_path)
        }
    }

    pub fn get_item<P: AsRef<Path>>(&self, path: P) -> Option<Item> {
        let fs_path = self.to_fs_path(path);
        if fs_path.is_none() {
            return None;
        }
        let fs_path = fs_path.unwrap();

        let name = fs_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let rel_path = fs_path.strip_prefix(&self.notes_path);
        if rel_path.is_err() {
            return None;
        }

        // skip non-.md files
        use std::ffi::OsStr;
        if fs_path.is_file() && fs_path.extension() != Some(OsStr::new("md")) {
            return None;
        }

        if fs_path.exists() {
            Some(Item {
                name: name,
                path: rel_path.unwrap().to_path_buf(),
                is_folder: fs_path.is_dir(),
                notebook: self,
            })
        } else {
            None
        }
    }
}

pub fn get_notebook<P: AsRef<Path>>(path: P) -> Option<Notebook> {
    let path = path.as_ref().canonicalize();
    if path.is_err() {
        return None;
    }
    let path = path.unwrap();

    if path.is_dir() {
        Some(Notebook {
            notebook_path: path.to_path_buf(),
            notes_path: {
                let mut p = path.to_path_buf();
                p.push("Notes");
                p
            },
        })
    } else {
        None
    }
}

#[test]
fn test_get_notebook() {
    assert_eq!(
        get_notebook("test").unwrap(),
        Notebook {
            notebook_path: PathBuf::from("test").canonicalize().unwrap(),
            notes_path: PathBuf::from("test/Notes").canonicalize().unwrap()
        }
    );
}

#[test]
fn test_get_notes() {
    let notebook = get_notebook("test").unwrap();
    let root = notebook.get_root().unwrap();
    let children = root.children();

    let subfolder = Item {
        name: "subfolder".into(),
        path: "subfolder".into(),
        is_folder: true,
        notebook: &notebook,
    };

    assert!(children.contains(&subfolder));
    assert_eq!(
        subfolder.children(),
        vec![
            Item {
                name: "hello".into(),
                path: "subfolder/hello.md".into(),
                is_folder: false,
                notebook: &notebook
            },
            Item {
                name: "subtest".into(),
                path: "subfolder/subtest.md".into(),
                is_folder: false,
                notebook: &notebook
            }
        ]
    );
}

#[allow(dead_code)]
fn recurse_notes(item: &Item, level: u64) {
    let children = item.children();

    for i in children {
        println!(
            "{}{}{}",
            {
                let mut s = String::new();
                for _ in 0..level {
                    s.push_str("  ");
                }
                s
            },
            i.name,
            match i.is_folder {
                true => ":",
                false => "",
            }
        );
        if i.is_folder {
            recurse_notes(&i, level + 1);
        }
    }
}
