use directories::{BaseDirs, ProjectDirs};
use std::env::var;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
use mocktopus::macros::mockable;

pub struct Config {
    prdir: ProjectDirs,
    pub ofi_tool: OfiTool,
    store_dirs: Vec<PathBuf>,
}

#[cfg_attr(test, mockable)]
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

    pub fn load(&mut self) -> Option<String> {
        if let Some(content) = self.read() {
            let mut lines = content.split('\n');
            let entry = lines.next().unwrap().to_string();
            if let Some(path) = lines.next() {
                self.rotate_to(path);
            }
            return Some(entry);
        }
        None
    }

    pub fn save(&self, entry: &str) -> std::io::Result<()> {
        self.write(&format!("{}\n{}", entry, self.get_path().display()))
    }

    fn read(&self) -> Option<String> {
        fs::read_to_string(self.latest_path()).ok()
    }

    fn write(&self, content: &str) -> std::io::Result<()> {
        if !self.prdir.data_dir().is_dir() {
            fs::create_dir_all(self.prdir.data_dir())?;
        }
        fs::write(self.latest_path(), content.as_bytes())
    }

    fn rotate_to(&mut self, store_path: &str) {
        if let Ok(pos) = self.store_dirs.binary_search(&PathBuf::from(store_path)) {
            self.store_dirs.rotate_left(pos);
        }
    }

    fn latest_path(&self) -> PathBuf {
        self.prdir.data_dir().join("latest")
    }
}

pub enum OfiTool {
    Wofi,
    Rofi,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mocktopus::mocking::*;

    #[test]
    fn load_old_latest() {
        Config::read.mock_safe(|_| MockResult::Return(Some("myentry".to_string())));
        let mut conf = Config::new();
        assert_eq!(Some("myentry".to_string()), conf.load());
    }

    #[test]
    fn no_previous_file() {
        Config::read.mock_safe(|_| MockResult::Return(None));
        let mut conf = Config::new();
        assert_eq!(None, conf.load());
    }

    #[test]
    fn load_latest() {
        Config::read
            .mock_safe(|_| MockResult::Return(Some("myentry\n/my/.password-store".to_string())));
        let mut conf = Config::new();
        assert_eq!(Some("myentry".to_string()), conf.load());
    }

    #[test]
    fn loading_same_store() {
        Config::read
            .mock_safe(|_| MockResult::Return(Some("myentry\n/my/.password-store".to_string())));
        let mut conf = Config::new();
        assert_eq!(Some("myentry".to_string()), conf.load());
    }

    #[test]
    fn save_latest() {
        Config::write.mock_safe(|_, entry| {
            assert_eq!("Foo\n/my/.password-store", entry);
            MockResult::Return(Ok(()))
        });
        let mut conf = Config::new();
        conf.store_dirs = vec![PathBuf::from("/my/.password-store")];
        assert_eq!((), conf.save("Foo").unwrap());
    }
}
