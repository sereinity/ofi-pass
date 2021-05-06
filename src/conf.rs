use directories::ProjectDirs;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn save<S: AsRef<str>>(entry: S) -> std::io::Result<()> {
    if let Some(prdir) = ProjectDirs::from("org", "sereinity", "ofi-pass") {
        if !prdir.data_dir().is_dir() {
            fs::create_dir_all(prdir.data_dir())?;
        }
        let mut file = File::create(prdir.data_dir().join("latest"))?;
        file.write(entry.as_ref().as_bytes())?;
    }
    Ok(())
}
