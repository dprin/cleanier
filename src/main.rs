use std::path::Path;

use config::Config;
use manager::Collector;
use package::Package;

mod config;
mod manager;
pub mod package;
mod utils;

fn main() {
    // TODO: add a default and custom parameter with clap
    let path: &Path = Path::new("./apps.toml");

    // TODO: generate collector from config
    let config = Config::read_config(path);
    let mut collector = Collector::new(config.manager);

    println!("{:#?}", collector.get_dependencies(Package::new("helix")));
    let dependency_graph = collector.system_dependency_graph();
    assert!(dependency_graph.iter().is_sorted());

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
