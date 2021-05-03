extern crate directories;
use directories::BaseDirs;
use std::{thread, time};

mod pass;
mod wofi;
mod wtype;

fn main() {
    if let Some(secret) = select_secret() {
        let action = select_action(&secret);
        match action {
            Action::PrintField(x) => wtype::wtype(secret.get(&x)),
            Action::Autotype => {
                let autovec = vec![
                    pass::EType::Field("user".to_string()),
                    pass::EType::Tab,
                    pass::EType::Field("pass".to_string()),
                ];
                for autoentry in autovec {
                    match autoentry {
                        pass::EType::Field(x) => wtype::wtype(secret.get(&x)),
                        pass::EType::Tab => {
                            wtype::wtype_key("tab");
                            thread::sleep(time::Duration::from_millis(12));
                        }
                        pass::EType::Enter => wtype::wtype_key("return"),
                        _ => todo!(),
                    }
                }
            }
            _ => {}
        }
    }
}

fn select_action(entry: &pass::PassEntry) -> Action {
    let fields = entry.list_fields();
    let selection = wofi::select(fields);
    match selection {
        Some(x) => match Some(x.as_str()) {
            Some("autotype") => Action::Autotype,
            Some(_) => Action::PrintField(x),
            _ => Action::Nothing,
        },
        _ => Action::Nothing,
    }
}

fn select_secret() -> Option<pass::PassEntry> {
    // List alternative roots and use via PASSWORD_STORE_DIR
    let dir_str = BaseDirs::new().expect("Can't determine directory structure");
    let store_dir = dir_str.home_dir().join(".password-store");
    // println!("PASSWORD_STORE_DIR: {}", store_dir.to_str()?);
    let store_dir = pass::PassDir::new(store_dir);
    wofi::select(store_dir.into_iter()).and_then(|x| Some(store_dir.show(&x)))
}

#[derive(Debug)]
enum Action {
    Autotype,
    PrintField(String),
    Nothing,
}
