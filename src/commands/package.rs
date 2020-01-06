use std::process::Command;

pub fn install(package: &str) {
    let installation_status = match Command::new("brew").arg("list").arg(package).output() {
        Ok(output) => output.status,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if installation_status.success() {
        println!("The {} package is already installed.", package)
    } else {
        println!("Installing the {} package.", package);
        match Command::new("brew").arg("install").arg(package).output() {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn upgrade(package: &str) {
    println!("Upgrading the {} package.", package);
    match Command::new("brew").arg("upgrade").arg(package).output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}
