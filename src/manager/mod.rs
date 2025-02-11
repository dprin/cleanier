use std::collections::{BTreeSet, HashMap, HashSet};

use pbr::ProgressBar;

use crate::package::Package;

pub mod pacman;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageManager {
    Pacman,
}

impl PackageManager {
    pub fn get_installed_packages(&self) -> BTreeSet<Package> {
        match self {
            PackageManager::Pacman => pacman::get_installed_packages(),

            // The main reason to allow this unreachable pattern is for
            // new implementations of package managers.
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }

    pub fn dependency_query(&self, package: &Package) -> HashSet<Package> {
        match self {
            PackageManager::Pacman => pacman::dependency_query(package),

            // The main reason to allow this unreachable pattern is for
            // new implementations of package managers.
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Collector {
    manager: PackageManager,
    // todo: comment
    dependency_map: HashMap<Package, HashSet<Package>>,
}

impl Collector {
    pub fn new(manager: PackageManager) -> Self {
        Self {
            manager,
            dependency_map: HashMap::new(),
        }
    }

    pub fn installed_packages(&self) -> BTreeSet<Package> {
        self.manager.get_installed_packages()
    }

    pub fn system_dependency_graph(&mut self) -> HashMap<Package, HashSet<Package>> {
        let packages = self.installed_packages();
        let amount = packages.len() as u64;
        let mut pb = ProgressBar::new(amount);

        for package in &packages {
            self.dependencies(package);
            pb.set(self.dependency_map.len() as u64);
        }

        pb.finish();
        self.dependency_map.clone()
    }

    pub fn dependencies(&mut self, package: &Package) -> HashSet<Package> {
        if self.dependency_map.contains_key(&package) {
            return HashSet::new();
        }

        // Make sure that there's something for no infinite loops.
        self.dependency_map.insert(package.clone(), HashSet::new());
        let mut package_deps = self.manager.dependency_query(&package);

        for d in package_deps.clone() {
            let extension = match self.dependency_map.get(&d) {
                Some(p) => p.clone(),
                None => self.dependencies(&d),
            };

            package_deps.extend(extension);
        }
        // insert the package itself as technically the package itself is also
        // its own dependency
        package_deps.insert(package.clone());

        self.dependency_map
            .insert(package.clone(), package_deps.clone());

        package_deps
    }
}
