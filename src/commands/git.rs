use std::path::PathBuf;
use std::process::Command;

pub fn clone(path: &PathBuf, url: &str) {
    println!("Cloning {} repo.", url);

    match Command::new("git")
        .current_dir(&path)
        .arg("clone")
        .arg(url)
        .output()
    {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}
