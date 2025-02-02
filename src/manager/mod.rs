use std::collections::{BTreeMap, BTreeSet};

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

    pub fn dependency_query(&self, package: &Package) -> BTreeSet<Package> {
        match self {
            PackageManager::Pacman => pacman::dependency_query(package),

            // The main reason to allow this unreachable pattern is for
            // new implementations of package managers.
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Collector {
    manager: PackageManager,
    dependencies: BTreeMap<Package, BTreeSet<Package>>,
}

impl Collector {
    pub fn new(manager: PackageManager) -> Self {
        Self {
            manager,
            dependencies: BTreeMap::new(),
        }
    }

    pub fn system_dependency_graph(&mut self) -> BTreeMap<Package, BTreeSet<Package>> {
        let packages = self.manager.get_installed_packages();

        for package in packages {
            self.get_dependencies(package);
        }

        self.dependencies.clone()
    }

    pub fn get_dependencies(&mut self, package: Package) -> BTreeSet<Package> {
        if self.dependencies.contains_key(&package) {
            return BTreeSet::new();
        }

        // Make sure that there's something for no infinite loops.
        self.dependencies.insert(package.clone(), BTreeSet::new());
        let mut package_deps = self.manager.dependency_query(&package);

        for d in package_deps.clone() {
            let extension = match self.dependencies.get(&d) {
                Some(p) => p.clone(),
                None => self.get_dependencies(d),
            };

            package_deps.extend(extension);
        }

        self.dependencies.insert(package, package_deps.clone());

        package_deps
    }
}
