extern crate directories;
use directories::BaseDirs;

mod pass;

fn main() {
    // List alternative roots and use via PASSWORD_STORE_DIR
    let dir_str = BaseDirs::new().expect("Can't determine directory structure");
    let store_dir = dir_str.home_dir().join(".password-store");
    println!("PASSWORD_STORE_DIR: {}", store_dir.to_str().unwrap());
    let store_dir = pass::PassDir::new(store_dir);
    for file in store_dir.into_iter() {
        println!("{}", file);
    }
}
