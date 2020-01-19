use crate::utils::printing;
use std::io::ErrorKind;
use std::process::Command;

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
