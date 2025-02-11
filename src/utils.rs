use core::str;
use std::{ffi::OsStr, process::Command};

//TODO: give output in error for more information
#[derive(Debug)]
pub enum CommandError {
    Execution,
    StringConversion,
}

pub fn run_command<'a>(
    command: impl AsRef<str> + AsRef<OsStr> + Into<&'a str>,
) -> Result<String, CommandError> {
    let command: &str = command.into();
    let command: Vec<&str> = command.split(" ").collect();

    let mut ret = Command::new(&command[0]);
    for arg in command.iter().skip(1) {
        ret.arg(arg);
    }

    let output = ret.output().map_err(|_| CommandError::Execution)?;
    let stringed = str::from_utf8(&output.stdout).map_err(|_| CommandError::StringConversion)?;

    Ok(stringed.to_string())
}
