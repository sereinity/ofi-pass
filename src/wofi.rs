use std::io::Write;
use std::process::{Command, Stdio};

pub fn select(input: impl Iterator<Item = String>) -> Option<String> {
    let child = Command::new("wofi")
        .args(&["--dmenu"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to exec wofi");
    for entry in input {
        child
            .stdin
            .as_ref()
            .and_then(|mut x| x.write(&(entry + "\n").into_bytes()).ok());
    }
    let output = child.wait_with_output().expect("wofi didn't ended well");
    String::from_utf8(output.stdout)
        .ok()
        .and_then(|x| Some(x.trim().into()))
}
