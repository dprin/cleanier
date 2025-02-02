use core::str;
use std::{ffi::OsStr, io::Error, process::Command};

#[derive(Debug)]
pub enum CommandError {
    Execution(Error),
    StringConversion(()),
}

pub fn run_command(command: &[impl AsRef<str> + AsRef<OsStr>]) -> Result<String, CommandError> {
    // TODO: This creates files for some reason. Fix.
    let mut ret = Command::new(&command[0]);
    for arg in command.iter().skip(1) {
        ret.arg(arg);
    }

    let output = ret.output().map_err(|err| CommandError::Execution(err))?;
    let stringed =
        str::from_utf8(&output.stdout).map_err(|_| CommandError::StringConversion(()))?;

    Ok(stringed.to_string())
}
