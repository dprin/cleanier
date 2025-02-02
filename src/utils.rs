use core::str;
use std::{ffi::OsStr, fmt::Display, process::Command};

pub fn run_command(command: impl AsRef<str> + Display + AsRef<OsStr>) -> String {
    // TODO: This creates files for some reason. Fix.

    let ret = Command::new("sh")
        .arg("-c")
        .arg(format!("{}", &command))
        .output()
        .expect(&format!("Failed to run {}", command));

    str::from_utf8(&ret.stdout)
        .expect(&format!("Could not convert output of {command} to string").to_string())
        .to_string()
}
