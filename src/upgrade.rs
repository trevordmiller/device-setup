use std::process::Command;

pub fn run() {
    // Environment (Unix)

    println!("Upgrading system package manager");
    Command::new("brew")
        .arg("update")
        .output()
        .expect("failed to execute process");
}
