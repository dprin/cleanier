use core::str;
use package::Package;
use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fmt::Display,
    fs,
    path::Path,
    process::Command,
    str::FromStr,
};

mod package;

fn run_command(command: impl AsRef<str> + Display + AsRef<OsStr>) -> String {
    let ret = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect(&format!("Failed to run {}", command).to_string());

    str::from_utf8(&ret.stdout)
        .expect(&format!("Could not convert output of {command} to string").to_string())
        .to_string()
}

fn generated_dependency_graph<'a>(
    packages: impl IntoIterator<Item = &'a Package>,
) -> Vec<(&'a Package, usize)> {
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

fn main() {
    let path: &Path = Path::new("./apps.txt");

    let command = run_command("pacman -Qe");
    let output = command.split('\n');
    let installed: HashSet<Package> = output
        .filter_map(|p| match Package::from_str(p) {
            Ok(p) => Some(p),
            _ => None,
        })
        .collect();

    let needed: Vec<Package> = fs::read_to_string(path)
        .expect("Could not access file.")
        .split('\n')
        .filter(|s| !s.is_empty() || s.starts_with('#'))
        .filter_map(|s| match Package::from_str(s) {
            Ok(p) => Some(p),
            _ => None,
        })
        .collect();

    let mut top_packages: HashMap<Package, usize> = HashMap::new();

    for p in &needed {
        for (package, value) in p.fetch_amount() {
            let amount = match top_packages.get(&package) {
                Some(prev) => value + prev,
                None => value,
            };

            top_packages.insert(package, amount);
        }
    }

    let mut sorted_packages: Vec<(&Package, &usize)> =
        top_packages.iter().map(|(k, v)| (k, v)).collect();

    sorted_packages.sort_by(|(_, a), (_, b)| a.cmp(b));

    let needed: HashSet<Package> = needed.iter().flat_map(|p| p.fetch_dependencies()).collect();
    let unneeded_packages: Vec<&Package> = installed.difference(&needed).collect();

    for d in generated_dependency_graph(unneeded_packages) {
        println!("{:?}: {:?}", d.0, d.1);
    }
}
