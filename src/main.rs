use std::{
    collections::{BTreeSet, HashSet},
    path::Path,
};

use config::Config;
use manager::Collector;
use package::Package;
use pbr::ProgressBar;

mod config;
mod manager;
pub mod package;
mod utils;

// separated to another function because i didn't like it being in the main >:(
fn process_required_packages(
    packages: &BTreeSet<Package>,
    collector: &mut Collector,
) -> HashSet<Package> {
    let mut progress_bar = ProgressBar::new(packages.len() as u64);

    packages
        .iter()
        .flat_map(|p| {
            progress_bar.message(format!("Finding dependencies for {} ", p.name).as_str());

            let ret = collector.dependencies(p);

            progress_bar.inc();
            return ret;
        })
        .collect()
}

fn main() {
    // TODO: add a default and custom parameter with clap
    let path: &Path = Path::new("./apps.toml");

    let config = Config::read_config(path);
    let mut collector = Collector::new(config.manager);

    println!("Fetching dependencies of required packages...");

    // required_packages -> all packages required + all dependencies
    let required_packages = process_required_packages(&config.required, &mut collector);

    println!("Fetching dependencies all installed packages...");

    // installed_packages -> all packages in the system
    let installed_packages = collector.system_dependency_graph();

    let dependency_count: BTreeSet<(usize, Package)> = installed_packages
        .into_iter()
        .filter_map(|(k, v)| {
            if required_packages.contains(&k) {
                return None;
            }

            let val = (v.difference(&required_packages).count(), k);
            return Some(val);
        })
        .collect();

    for (k, v) in dependency_count {
        println!("{v}: {k}");
    }
}
