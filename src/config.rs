use crate::{manager::PackageManager, package::Package};
use serde::Deserialize;
use std::{
    collections::{BTreeSet, HashMap},
    fs,
    path::Path,
};

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Config {
    pub required: BTreeSet<Package>,
    pub manager: PackageManager,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum PackageItems {
    Category(HashMap<String, Box<PackageItems>>),
    Packages(Vec<String>),
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct ParsedConfig {
    apps: PackageItems,
    manager: String,
}

fn parse_package(s: &str) -> Package {
    Package::new(s)
}

impl PackageItems {
    fn flatten(self) -> Vec<String> {
        match self {
            PackageItems::Category(hash_map) => hash_map
                .iter()
                .map(|(_, v)| v.clone().flatten()) // FIXME: I don't like this clone
                .flatten()
                .collect(),
            PackageItems::Packages(items) => items,
        }
    }
}

impl ParsedConfig {
    fn generate_config(self) -> Result<Config, String> {
        let packages: BTreeSet<Package> = self
            .apps
            .flatten()
            .iter()
            .map(|name| parse_package(name))
            .collect();

        let manager = match self.manager.as_str() {
            "pacman" => PackageManager::Pacman,
            _ => return Err(String::from("why")),
        };

        Ok(Config {
            manager,
            required: packages,
        })
    }
}

impl Config {
    pub fn read_config(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let parsed: ParsedConfig = toml::from_str(content.as_str()).unwrap();

        parsed
            .generate_config()
            .expect("Was not able to generate config")
    }
}
