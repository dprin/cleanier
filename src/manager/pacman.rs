use crate::{package::Package, utils::run_command};
use std::collections::{BTreeSet, HashSet};

fn parse_package(s: &str) -> Result<Package, ()> {
    let split: Vec<&str> = s.split(' ').collect();

    match split.len() {
        1 | 2 => {
            if split[0].is_empty() {
                return Err(());
            }

            let name: String = split[0]
                .split_terminator(&['>', '=', '<'])
                .nth(0)
                .unwrap() // SAFETY: we know there is at least something in the iterator
                .to_string();

            if name.contains(".so") {
                return Err(());
            }
            return Ok(Package::new(name));
        }

        _ => Err(()),
    }
}

pub fn dependency_query(package: &Package) -> HashSet<Package> {
    let command = format!("pacman -Qi {}", &package.name);
    let search = run_command(command.as_str()).unwrap();

    search
        .split('\n')
        .filter(|s| s.contains("Depends On")) // Find the line that contains dependencies
        .map(|s| s.split(':').nth(1).unwrap()) // Remove the "Depends On :" Section
        .filter(|s| !s.contains("None")) // If there's no dependencies, remove the line
        .flat_map(|s| s.split(' ').collect::<Vec<&str>>())
        .filter(|s| !s.is_empty()) // Split up to package names
        .filter_map(|p| match parse_package(p) {
            Ok(p) => Some(p),
            _ => None,
        }) // extract as packages
        .collect()
}

pub fn get_installed_packages() -> BTreeSet<Package> {
    let command = run_command("pacman -Q").unwrap();
    let output = command.split('\n');

    output
        .filter_map(|p| match parse_package(p) {
            Ok(p) => Some(p),
            _ => None,
        })
        .collect()
}
