use std::{collections::BTreeSet, fs, path::Path};

use crate::{
    manager::PackageManager,
    package::{Package, Parseable},
};

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Config {
    required: BTreeSet<Package>,
    manager: PackageManager,
}

impl Parseable for Config {
    fn parse_package(s: &str) -> Result<Package, ()> {
        Ok(Package::new(s))
    }
}

impl Config {
    pub fn read_config(path: &Path) -> Self {
        let required: BTreeSet<Package> = fs::read_to_string(path)
            .expect("Could not access file.")
            .split('\n')
            .filter(|s| !s.is_empty() || s.starts_with('#'))
            .filter_map(|s| match Self::parse_package(s) {
                Ok(p) => Some(p),
                _ => None,
            })
            .collect();

        Self {
            required,
            manager: PackageManager::Pacman,
        }
    }
}
