use super::Manager;
use crate::{package::Package, utils::run_command};
use std::{collections::BTreeSet, str::FromStr};

type PacPackage = Package<Pacman>;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Pacman {}

impl Manager for Pacman {
    fn dependency_query(&self, package: &PacPackage) -> BTreeSet<PacPackage> {
        let search = run_command(format!("pacman -Qi {}", &package.name));

        search
            .split('\n')
            .filter(|s| s.contains("Depends On")) // Find the line that contains dependencies
            .map(|s| s.split(':').nth(1).unwrap()) // Remove the "Depends On :" Section
            .filter(|s| !s.contains("None")) // If there's no dependencies, remove the line
            .flat_map(|s| s.split(' ').collect::<Vec<&str>>())
            .filter(|s| !s.is_empty()) // Split up to package names
            .filter_map(|p| match Package::from_str(p) {
                Ok(p) => Some(p),
                _ => None,
            }) // extract as packages
            .collect()
    }

    fn get_installed_packages() -> BTreeSet<PacPackage> {
        let command = run_command("pacman -Qe");
        let output = command.split('\n');

        output
            .filter_map(|p| match Package::from_str(p) {
                Ok(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    fn parse_package(s: &str) -> Result<Package<Self>, ()> {
        let split: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();

        match split.len() {
            1 | 2 => {
                if split[0].is_empty() {
                    return Err(());
                }

                return Ok(Package::new(split[0].clone()));
            }

            _ => Err(()),
        }
    }
}
