use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub struct PassDir {
    pub root: PathBuf,
}

impl<'a> PassDir {
    pub fn new(root: PathBuf) -> PassDir {
        PassDir { root }
    }

    pub fn into_iter(self: &'a Self) -> impl Iterator<Item = String> + 'a {
        WalkDir::new(self.root.clone())
            .min_depth(1)
            .into_iter()
            .filter_entry(|x| !is_hidden(x))
            .filter_map(move |x| clean_name(x.ok(), &self.root))
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn clean_name(entry: Option<DirEntry>, prefix: &PathBuf) -> Option<String> {
    match entry {
        Some(x) => x
            .path()
            .strip_prefix(prefix)
            .ok()
            .and_then(|y| y.to_str())
            .and_then(|y| y.strip_suffix(".gpg"))
            .and_then(|y| Some(y.to_string())),
        _ => None,
    }
}
