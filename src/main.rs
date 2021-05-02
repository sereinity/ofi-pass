extern crate directories;
use directories::BaseDirs;

mod pass;
mod wofi;

fn main() {
    println!(
        "{}",
        select_secret().expect("Failed to query the desired secret")
    );
}

fn select_secret() -> Option<String> {
    // List alternative roots and use via PASSWORD_STORE_DIR
    let dir_str = BaseDirs::new().expect("Can't determine directory structure");
    let store_dir = dir_str.home_dir().join(".password-store");
    // println!("PASSWORD_STORE_DIR: {}", store_dir.to_str()?);
    let store_dir = pass::PassDir::new(store_dir);
    wofi::select(store_dir.into_iter())
}
