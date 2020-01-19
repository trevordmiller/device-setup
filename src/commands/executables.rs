use crate::utils::printing;
use std::io::ErrorKind;
use std::process::Command;

pub fn run(executable: &str) {
    printing::progress(format!("Running {}.", executable));
    let path_check = match Command::new("which").arg(executable).output() {
        Ok(output) => output.stdout,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if path_check.is_empty() {
        match Command::new(executable).output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The {} executable is missing.", executable),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    }
}

pub fn update(executable: &str) {
    printing::progress(format!("Updating {}.", executable));
    match Command::new(executable).arg("update").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The {} executable is missing.", executable),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }
}
