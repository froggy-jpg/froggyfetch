use std::process::{Command, Stdio};

pub fn fetch_packages() -> String {
    let mut child1 = Command::new("pacman")
        .arg("-Qq")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let child2 = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::from(child1.stdout.take().unwrap()))
        .output()
        .unwrap();
    let package_count = String::from_utf8(child2.stdout).unwrap();
    package_count.replace('\n', "")
}
