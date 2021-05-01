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
            .filter_map(move |x| clean_name(x.unwrap(), &self.root))
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn clean_name(entry: DirEntry, prefix: &PathBuf) -> Option<String> {
    entry
        .path()
        .strip_prefix(prefix)
        .ok()
        .and_then(|x| x.to_str())
        .and_then(|x| x.strip_suffix(".gpg"))
        .and_then(|x| Some(x.to_string()))
}
