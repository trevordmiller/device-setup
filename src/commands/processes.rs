use std::process::Command;

pub fn stop(program: &str) {
    println!("Quitting {} processes.", program);
    match Command::new("killall").arg("rls").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}
