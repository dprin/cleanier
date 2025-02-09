use std::path::Path;

use config::Config;
use manager::Collector;

mod config;
mod manager;
pub mod package;
mod utils;

fn main() {
    // TODO: add a default and custom parameter with clap
    let path: &Path = Path::new("./apps.toml");

    let config = Config::read_config(path);
    let mut collector = Collector::new(config.manager);
    let amount = collector.installed_packages().len();

    println!("amount of installed packages: {amount}");
    let _ = collector.system_dependency_graph();
}
