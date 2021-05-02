use directories::BaseDirs;
use std::process::Command;

pub fn wtype(text: &String) {
    let base_dirs = BaseDirs::new().expect("Can't determine directory structure");
    let wtype_bin = base_dirs.home_dir().join("Projects/wtype/build/wtype");
    Command::new(wtype_bin)
        .args(&[text])
        .spawn()
        .expect("Failed to call wtype");
}
