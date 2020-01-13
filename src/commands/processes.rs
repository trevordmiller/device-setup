use crate::utils::printing;
use std::process::Command;

pub fn stop(program: &str) {
    printing::progress(format!("Quitting {} processes.", program));
    match Command::new("killall").arg("rls").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}
