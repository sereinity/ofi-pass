use directories::{BaseDirs, ProjectDirs};
use std::env::var;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    prdir: ProjectDirs,
    pub ofi_tool: OfiTool,
    store_dirs: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        let dir_str = BaseDirs::new().expect("Can't determine directory structure");
        Config {
            prdir: ProjectDirs::from("org", "sereinity", "ofi-pass")
                .expect("Can't guess config pass"),
            ofi_tool: match var("OFI_TOOL") {
                Ok(x) if x.eq("wofi") => OfiTool::Wofi,
                _ => OfiTool::Rofi,
            },
            // List alternative roots and use via PASSWORD_STORE_DIR
            store_dirs: vec![dir_str.home_dir().join(".password-store")],
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self
            .store_dirs
            .first()
            .expect("Should have a default pass store")
    }

    pub fn load(&self) -> Option<String> {
        fs::read_to_string(self.latest_path()).ok()
    }

    pub fn save(&self, entry: &str) -> std::io::Result<()> {
        if !self.prdir.data_dir().is_dir() {
            fs::create_dir_all(self.prdir.data_dir())?;
        }
        fs::write(self.latest_path(), entry.as_bytes())
    }

    fn latest_path(&self) -> PathBuf {
        self.prdir.data_dir().join("latest")
    }
}

pub enum OfiTool {
    Wofi,
    Rofi,
}
