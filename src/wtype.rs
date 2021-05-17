use std::process::Command;

pub fn wtype(text: &String) {
    Command::new("wtype")
        .args(&[text])
        .spawn()
        .expect("Failed to call wtype");
}

pub fn wtype_key<S: AsRef<str>>(keyname: S) {
    Command::new("wtype")
        .args(&["-k", keyname.as_ref()])
        .spawn()
        .expect("Failed to call wtype");
}
