use std::process::Command;

pub fn shell(cmd: String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd).output().unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    out
}