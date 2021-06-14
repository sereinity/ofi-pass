extern crate directories;
use std::{thread, time};

mod conf;
mod pass;
mod rofi;
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
                            thread::sleep(time::Duration::from_millis(250));
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
    let pass_store = pass::PassDir::new(config.get_path().clone());
    config
        .ofi_tool
        .select(pass_store.iter(), config.load())
        .and_then(|x| pass_store.show(&x))
        .and_then(|x| config.save(x.get_name()).ok().and(Some(x)))
}

impl conf::OfiTool {
    pub fn select<I, T>(&self, input: I, default: Option<T>) -> Option<String>
    where
        I: Iterator<Item = T>,
        T: AsRef<str>,
    {
        match self {
            conf::OfiTool::Wofi => wofi::select(input, default),
            conf::OfiTool::Rofi => rofi::select(input, default),
        }
    }
}

enum Action {
    Autotype,
    PrintField(String),
    Nothing,
}
