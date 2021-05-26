use std::process::Command;

pub fn wtype(text: &str) {
    Command::new("wtype")
        .args(&[text])
        .status()
        .expect("Failed to call wtype");
}

pub fn wtype_key(keyname: &str) {
    Command::new("wtype")
        .args(&["-k", keyname])
        .status()
        .expect("Failed to call wtype");
}
