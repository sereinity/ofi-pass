use directories::ProjectDirs;
use std::env::var;
use std::fs::{self, File};
use std::io::prelude::*;

pub struct Config {
    prdir: ProjectDirs,
    pub ofi_tool: OfiTool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            prdir: ProjectDirs::from("org", "sereinity", "ofi-pass")
                .expect("Can't guess config pass"),
            ofi_tool: match var("OFI_TOOL") {
                _ => OfiTool::Wofi,
            },
        }
    }

    pub fn load(&self) -> Option<String> {
        let rfile = File::open(self.prdir.data_dir().join("latest"));
        if let Ok(mut file) = rfile {
            let mut content = String::new();
            file.read_to_string(&mut content).ok().and(Some(content))
        } else {
            None
        }
    }

    pub fn save<S: AsRef<str>>(&self, entry: S) -> std::io::Result<()> {
        if !self.prdir.data_dir().is_dir() {
            fs::create_dir_all(self.prdir.data_dir())?;
        }
        let mut file = File::create(self.prdir.data_dir().join("latest"))?;
        file.write(entry.as_ref().as_bytes())?;
        Ok(())
    }
}

pub enum OfiTool {
    Wofi,
}
