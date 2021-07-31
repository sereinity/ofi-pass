use std::io::Write;
use std::process::{Command, Stdio};

pub fn select<I, T>(input: I, default: Option<T>) -> Option<String>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut cmd = Command::new("rofi");
    cmd.args(&["-dmenu", "-i", "-p", ">"]);
    if let Some(entry) = default {
        cmd.args(&["-select", entry.as_ref()]);
    }
    let child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to exec rofi");
    for entry in input {
        let line = entry.as_ref().to_owned() + "\n";
        child
            .stdin
            .as_ref()
            .and_then(|mut x| x.write(&(line).into_bytes()).ok());
    }
    let output = child.wait_with_output().expect("rofi didn't ended well");
    String::from_utf8(output.stdout)
        .ok()
        .map(|x| x.trim().into())
        .filter(|x: &String| !x.is_empty())
}
