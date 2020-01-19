use crate::utils::printing;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn clone(path: &PathBuf, url: &str) {
    printing::progress(format!("Cloning {} repo.", url));

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

pub fn pull_all(path: &PathBuf) {
    printing::progress(format!(
        "Upgrading all repos in {}.",
        path.to_string_lossy()
    ));
    match fs::read_dir(path) {
        Ok(dirs) => {
            for dir in dirs {
                match Command::new("git")
                    .current_dir(match dir {
                        Ok(dir) => dir.path(),
                        Err(error) => panic!("There was a problem: {:?}", error),
                    })
                    .arg("pull")
                    .output()
                {
                    Ok(_) => (),
                    Err(error) => panic!("There was a problem: {:?}", error),
                }
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}

pub fn check_all(path: &PathBuf) {
    printing::progress(format!("Checking all repos in {}.", path.to_string_lossy()));
    match fs::read_dir(path) {
        Ok(dirs) => {
            let mut all_repos_clean = true;

            for dir in dirs {
                let dir = match &dir {
                    Ok(dir) => dir.path(),
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let status_check = match Command::new("git")
                    .current_dir(&dir)
                    .arg("status")
                    .arg("--porcelain")
                    .output()
                {
                    Ok(output) => output.stdout,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let unpushed_check = match Command::new("git")
                    .current_dir(&dir)
                    .arg("log")
                    .arg("@{u}..")
                    .output()
                {
                    Ok(output) => output.stdout,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                if !status_check.is_empty() || !unpushed_check.is_empty() {
                    all_repos_clean = false;
                    printing::error(format!("A dirty repo was found: {}", &dir.display()));
                }
            }

            if !all_repos_clean {
                panic!("Repos are dirty.");
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}
