use std::path::Path;

mod manager;
pub mod package;
mod utils;

fn main() {
    // TODO: add a default and custom parameter with clap
    let path: &Path = Path::new("./apps.txt");

    // TODO: scan from config

    // let command = run_command("pacman -Qe");
    // let output = command.split('\n');
    // let installed: HashSet<Package> = output
    //     .filter_map(|p| match Package::from_str(p) {
    //         Ok(p) => Some(p),
    //         _ => None,
    //     })
    //     .collect();

    // let needed: Vec<Package> = fs::read_to_string(path)
    //     .expect("Could not access file.")
    //     .split('\n')
    //     .filter(|s| !s.is_empty() || s.starts_with('#'))
    //     .filter_map(|s| match Package::from_str(s) {
    //         Ok(p) => Some(p),
    //         _ => None,
    //     })
    //     .collect();

    // let mut top_packages: HashMap<Package, usize> = HashMap::new();

    // for p in &needed {
    //     for (package, value) in p.fetch_amount() {
    //         let amount = match top_packages.get(&package) {
    //             Some(prev) => value + prev,
    //             None => value,
    //         };

    //         top_packages.insert(package, amount);
    //     }
    // }

    // let mut sorted_packages: Vec<(&Package, &usize)> =
    //     top_packages.iter().map(|(k, v)| (k, v)).collect();

    // sorted_packages.sort_by(|(_, a), (_, b)| a.cmp(b));

    // let needed: HashSet<Package> = needed.iter().flat_map(|p| p.fetch_dependencies()).collect();
    // let unneeded_packages: Vec<&Package> = installed.difference(&needed).collect();

    // for d in generated_dependency_graph(unneeded_packages.clone()) {
    //     println!("{:?}: {:?}", d.0, d.1);
    // }

    // println!("Amount of unneeded packages: {}", &unneeded_packages.len());
}
