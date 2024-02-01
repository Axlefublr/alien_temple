use std::process::Command;

use crate::extra::WORKING_DIR;

pub fn git_add_commit(message: &str) -> Result<(), &'static str> {
    if Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(WORKING_DIR)
        .output()
        .is_err()
    {
        return Err("couldn't git add");
    }
    if Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(WORKING_DIR)
        .output()
        .is_err()
    {
        return Err("couldn't git commit");
    }
    Ok(())
}
