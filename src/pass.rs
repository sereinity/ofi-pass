use log::{error, info};
use std::collections::HashMap;
use std::env::var;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

pub struct PassDir {
    pub root: PathBuf,
}

impl<'a> PassDir {
    pub fn new(root: PathBuf) -> PassDir {
        PassDir { root }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = String> + 'a {
        WalkDir::new(self.root.clone())
            .min_depth(1)
            .into_iter()
            .filter_entry(|x| !is_hidden(x))
            .filter_map(move |x| clean_name(x.ok(), &self.root))
    }

    pub fn show(self, entry: &str) -> Option<PassEntry> {
        PassEntry::from_pass(entry)
    }
}

pub struct PassEntry {
    name: String,
    values: HashMap<String, String>,
}

impl PassEntry {
    fn new(name: String) -> Self {
        let mut values = HashMap::new();
        values.insert(":autotype".to_string(), "user :tab pass".to_string());
        values.insert(
            "user".to_string(),
            var("USER").expect("Couldn't get OS username"),
        );
        PassEntry { name, values }
    }

    pub fn from_pass(entry_name: &str) -> Option<Self> {
        let mut entry = Self::new(entry_name.to_string());
        let output = Command::new("pass")
            .args(["show", entry_name])
            .output()
            .expect("fail to exec pass");
        if !output.status.success() {
            return None;
        }
        let fullout = std::str::from_utf8(&output.stdout).unwrap();
        parse_entry_string(fullout, &mut entry);
        Some(entry)
    }

    pub fn list_fields(&self) -> impl Iterator<Item = &String> {
        let mut keys = self.values.keys().collect::<Vec<&String>>();
        keys.sort();
        keys.into_iter()
    }

    pub fn get(&self, field_name: &str) -> &String {
        self.values
            .get(field_name)
            .expect("Getting a value absent of the entry")
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn gen_otp(&self) -> String {
        let output = Command::new("sh")
            .args(["-c", &self.values[":otp"]])
            .output()
            .expect("fail to exec pass-otp");
        if !output.status.success() {
            error!("Cannot run pass-otp");
            return "".to_string();
        }
        std::str::from_utf8(&output.stdout)
            .unwrap()
            .trim_end()
            .to_string()
    }

    pub fn autoseq(&self) -> Vec<EType> {
        let mut seq = vec![];
        for word in self.get(":autotype").split_whitespace() {
            seq.push(match Some(word) {
                Some(":tab") => EType::Tab,
                Some(":enter") => EType::Enter,
                Some(":space") => EType::Space,
                Some(":delay") => EType::Delay,
                Some(":otp") => EType::Otp,
                Some("path") => EType::Path,
                Some(x) => EType::Field(x.to_string()),
                None => panic!("Impossible case"),
            });
        }
        seq
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn parse_entry_string(content: &str, entry: &mut PassEntry) {
    let splitted_out = content.split('\n');
    let mut lines = splitted_out.map(|x| x.to_string());
    entry
        .values
        .insert("pass".to_string(), lines.next().unwrap());
    for extra in lines {
        match extra.split_once(": ") {
            Some(("autotype", value)) => entry
                .values
                .insert(":autotype".to_string(), value.to_string()),
            Some(("otp_method", value)) => {
                entry.values.insert(":otp".to_string(), value.to_string())
            }
            Some((label, value)) => entry.values.insert(label.to_string(), value.to_string()),
            None => {
                if extra.starts_with("otpauth://") {
                    let mut cmd = "pass otp ".to_string();
                    cmd.push_str(&entry.name);
                    entry.values.insert(":otp".to_string(), cmd)
                } else {
                    info!("Parsing a non splittable line '{}'", extra);
                    continue;
                }
            }
        };
    }
}

fn clean_name(entry: Option<DirEntry>, prefix: &Path) -> Option<String> {
    match entry {
        Some(x) => x
            .path()
            .strip_prefix(prefix)
            .ok()
            .and_then(|y| y.to_str())
            .and_then(|y| y.strip_suffix(".gpg"))
            .map(|y| y.to_string()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut entry = PassEntry::new("My entry".to_string());
        parse_entry_string("", &mut entry);
        assert_eq!(entry.name, "My entry");
        assert_eq!(entry.values.keys().len(), 3);
        assert_eq!(entry.values[":autotype"], "user :tab pass");
        assert_eq!(entry.values["pass"], "");
        assert_ne!(entry.values["user"], "");
    }

    #[test]
    fn only_pass() {
        let mut entry = PassEntry::new("My entry".to_string());
        parse_entry_string("my: password", &mut entry);
        assert_eq!(entry.values.keys().len(), 3);
        assert_eq!(entry.values["pass"], "my: password");
        assert_eq!(entry.values.get(":otp"), None);
    }

    #[test]
    fn with_extra_newline() {
        let mut entry = PassEntry::new("My entry".to_string());
        parse_entry_string(
            "my: password\n\
            user: foo\n\n\n\
            custom: bar",
            &mut entry,
        );
        assert_eq!(entry.values.keys().len(), 4);
        assert_eq!(entry.values["pass"], "my: password");
        assert_eq!(entry.values["user"], "foo");
        assert_eq!(entry.values["custom"], "bar");
    }

    #[test]
    fn full_file() {
        let mut entry = PassEntry::new("myEntry".to_string());
        parse_entry_string(
            "my: password\n\
            autotype: user :enter pass\n\
            user: foo\n\
            otpauth://topt/my?secret=foo&bar=baz\n\
            custom: bar",
            &mut entry,
        );
        assert_eq!(entry.values.keys().len(), 5);
        assert_eq!(entry.values[":autotype"], "user :enter pass");
        assert_eq!(entry.values["pass"], "my: password");
        assert_eq!(entry.values["user"], "foo");
        assert_eq!(entry.values[":otp"], "pass otp myEntry");
        assert_eq!(entry.values["custom"], "bar");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn multiline() {
        let mut entry = PassEntry::new("My entry".to_string());
        parse_entry_string(
            "my: password\n\
            \n\
            ---\n\
            user: foo",
            &mut entry,
        );
        assert_eq!(entry.values.keys().len(), 3);
        assert_eq!(entry.values["pass"], "my: password\n");
        assert_eq!(entry.values["user"], "foo");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn follow_file() {
        let mut entry = PassEntry::new("My entry".to_string());
        parse_entry_string(
            "#FILE=my_file\n\
            user: foo",
            &mut entry,
        );
        assert_eq!(entry.values.keys().len(), 3);
        assert_eq!(entry.values["pass"], "#FILE=my_file");
        assert_eq!(entry.values["user"], "foo");
        todo!("test entry.pass() would load the file");
    }

    #[test]
    fn otp_command() {
        let mut entry = PassEntry::new("My entry".to_string());
        parse_entry_string(
            "mypass\n\
            otp_method: my command",
            &mut entry,
        );
        assert_eq!(entry.values.keys().len(), 4);
        assert_eq!(entry.values["pass"], "mypass");
        assert_eq!(entry.values[":otp"], "my command");
    }

    #[test]
    fn pass_otp() {
        let mut entry = PassEntry::new("MyEntry".to_string());
        parse_entry_string(
            "mypass\n\
            otpauth://topt/my?secret=foo&bar=baz\n",
            &mut entry,
        );
        assert_eq!(entry.values.keys().len(), 4);
        assert_eq!(entry.values["pass"], "mypass");
        assert_eq!(entry.values[":otp"], "pass otp MyEntry");
    }
}
