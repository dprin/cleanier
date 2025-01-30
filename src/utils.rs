use core::str;
use std::{ffi::OsStr, fmt::Display, process::Command};

use crate::package::Package;

pub fn run_command(command: impl AsRef<str> + Display + AsRef<OsStr>) -> String {
    let ret = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect(&format!("Failed to run {}", command).to_string());

    str::from_utf8(&ret.stdout)
        .expect(&format!("Could not convert output of {command} to string").to_string())
        .to_string()
}

pub fn generated_dependency_graph<'a, M>(
    packages: impl IntoIterator<Item = &'a Package<M>>,
) -> Vec<(&'a Package<M>, usize)> {
    let mut dependency_graph: Vec<_> = packages
        .into_iter()
        .map(|x| {
            let amount = x.fetch_dependencies().len();
            (x, amount)
        })
        .collect();

    dependency_graph.sort_by(|a, b| a.1.cmp(&b.1));
    dependency_graph
}
