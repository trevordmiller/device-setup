use crate::utils::printing;
use std::io::ErrorKind;
use std::process::Command;

pub fn upgrade_self() {
    printing::progress("Upgrading rust.".to_string());
    match Command::new("rustup").arg("update").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The rustup executable is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }
}

pub fn upgrade_executable(executable: &str) {
    printing::progress(format!("Upgrading {}.", executable));
    match Command::new("cargo")
        .arg("install")
        .arg(executable)
        .output()
    {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The {} executable is missing.", executable),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }
}
