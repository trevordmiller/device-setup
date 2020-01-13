use crate::utils::printing;
use std::io::ErrorKind;
use std::process::Command;

pub fn install_package(package: &str, after_install: &dyn Fn()) {
    let installation_status = match Command::new("brew").arg("list").arg(package).output() {
        Ok(output) => output.status,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if installation_status.success() {
        printing::info(format!("The {} package is already installed.", package));
    } else {
        printing::progress(format!("Installing the {} package.", package));
        match Command::new("brew").arg("install").arg(package).output() {
            Ok(_) => after_install(),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn upgrade_package(package: &str) {
    printing::progress(format!("Upgrading the {} package.", package));
    match Command::new("brew").arg("upgrade").arg(package).output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

pub fn upgrade_rust() {
    printing::progress("Upgrading rust.".to_string());
    match Command::new("rustup").arg("update").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The rustup command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }
}

pub fn upgrade_self() {
    printing::progress("Upgrading homebrew.".to_string());
    match Command::new("brew").arg("update").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}

pub fn clean_artifacts() {
    printing::progress("Removing homebrew artifacts.".to_string());
    match Command::new("brew").arg("cleanup").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}
