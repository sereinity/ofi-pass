use std::process::{Command, Stdio};
extern crate directories;
use directories::BaseDirs;
use std::io::Write;

mod pass;

fn main() {
    select_secret();
}

fn select_secret() {
    // List alternative roots and use via PASSWORD_STORE_DIR
    let dir_str = BaseDirs::new().expect("Can't determine directory structure");
    let store_dir = dir_str.home_dir().join(".password-store");
    // println!("PASSWORD_STORE_DIR: {}", store_dir.to_str().unwrap());
    let store_dir = pass::PassDir::new(store_dir);

    let child = Command::new("wofi")
        .args(&["--dmenu"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to exec wofi");
    for file in store_dir.into_iter() {
        child
            .stdin
            .as_ref()
            .and_then(|mut x| x.write(&(file + "\n").into_bytes()).ok());
    }
    let output = child.wait_with_output().expect("wofi didn't ended well");
    println!("{}", String::from_utf8(output.stdout).unwrap().trim());
}
