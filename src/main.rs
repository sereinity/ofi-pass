extern crate directories;
use directories::BaseDirs;
use std::{thread, time};

mod conf;
mod pass;
mod wofi;
mod wtype;

fn main() {
    let config = conf::Config::new();
    if let Some(secret) = select_secret(&config) {
        let action = select_action(&config, &secret);
        match action {
            Action::PrintField(x) => wtype::wtype(secret.get(&x)),
            Action::Autotype => {
                for autoentry in secret.autoseq() {
                    match autoentry {
                        pass::EType::Field(x) => wtype::wtype(secret.get(&x)),
                        pass::EType::Tab => {
                            wtype::wtype_key("tab");
                            thread::sleep(time::Duration::from_millis(12));
                        }
                        pass::EType::Enter => wtype::wtype_key("return"),
                        pass::EType::Space => wtype::wtype_key("space"),
                        pass::EType::Path => wtype::wtype(secret.get_name()),
                        pass::EType::Delay => thread::sleep(time::Duration::from_millis(200)),
                        pass::EType::Otp => todo!(),
                    }
                }
            }
            _ => {}
        }
    }
}

fn select_action(config: &conf::Config, entry: &pass::PassEntry) -> Action {
    let fields = entry.list_fields();
    match config.ofi_tool.select(fields, None) {
        Some(x) => match Some(x.as_str()) {
            Some("autotype") => Action::Autotype,
            Some(_) => Action::PrintField(x),
            _ => Action::Nothing,
        },
        _ => Action::Nothing,
    }
}

fn select_secret(config: &conf::Config) -> Option<pass::PassEntry> {
    // List alternative roots and use via PASSWORD_STORE_DIR
    let dir_str = BaseDirs::new().expect("Can't determine directory structure");
    let store_dir = dir_str.home_dir().join(".password-store");
    // println!("PASSWORD_STORE_DIR: {}", store_dir.to_str()?);
    let store_dir = pass::PassDir::new(store_dir);
    config
        .ofi_tool
        .select(store_dir.into_iter(), config.load())
        .and_then(|x| store_dir.show(&x))
        .and_then(|x| config.save(x.get_name()).ok().and(Some(x)))
}

impl conf::OfiTool {
    pub fn select<I, T>(&self, input: I, default: Option<T>) -> Option<String>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        match self {
            conf::OfiTool::Wofi => wofi::select(input, default),
        }
    }
}

enum Action {
    Autotype,
    PrintField(String),
    Nothing,
}
