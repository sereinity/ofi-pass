use std::collections::HashMap;
use std::env::var;
use std::path::PathBuf;
use std::process::Command;
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

    pub fn show(self, entry: &String) -> PassEntry {
        PassEntry::from_pass(entry)
    }
}

#[derive(Debug)]
pub struct PassEntry {
    name: String,
    values: HashMap<String, String>,
}

impl PassEntry {
    fn new(name: String) -> Self {
        let mut values = HashMap::new();
        values.insert("autotype".to_string(), "user :tab pass".to_string());
        values.insert(
            "user".to_string(),
            var("USER").expect("Couldn't get OS username"),
        );
        PassEntry { name, values }
    }

    pub fn from_pass(entry_name: &String) -> Self {
        let mut entry = Self::new(entry_name.to_string());
        let output = Command::new("pass")
            .args(&["show", entry_name])
            .output()
            .expect("fail to exec pass");
        let fullout = String::from_utf8(output.stdout).unwrap();
        let splitted_out = fullout.split('\n');
        let mut lines = splitted_out.map(|x| x.to_string());
        entry
            .values
            .insert("pass".to_string(), lines.next().unwrap());
        for extra in lines {
            match extra.split_once(':') {
                Some((label, value)) => entry
                    .values
                    .insert(label.to_string(), value.trim_start().to_string()),
                None => continue,
            };
        }
        entry
    }

    pub fn list_fields(&self) -> impl Iterator<Item = &String> {
        let mut keys = self.values.keys().collect::<Vec<&String>>();
        keys.sort();
        keys.into_iter()
    }

    pub fn get<T: AsRef<str>>(&self, field_name: T) -> &String {
        self.values
            .get(field_name.as_ref())
            .expect("Getting a value absent of the entry")
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

pub enum EType {
    Field(String),
    Tab,
    Space,
    Delay,
    Enter,
    Otp,
    Path,
}
