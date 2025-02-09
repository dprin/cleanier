use std::collections::{BTreeMap, BTreeSet};

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
    // todo: comment
    dependency_map: BTreeMap<Package, BTreeSet<Package>>,
}

impl Collector {
    pub fn new(manager: PackageManager) -> Self {
        Self {
            manager,
            dependency_map: BTreeMap::new(),
        }
    }

    pub fn installed_packages(&self) -> BTreeSet<Package> {
        self.manager.get_installed_packages()
    }

    pub fn system_dependency_graph(&mut self) -> BTreeMap<Package, BTreeSet<Package>> {
        let packages = self.installed_packages();
        let amount = packages.len() as u64;
        let mut pb = ProgressBar::new(amount);

        for package in &packages {
            self.dependencies(package);
            pb.set(self.dependency_map.len() as u64);
        }

        pb.finish();
        let found_packages: BTreeSet<Package> =
            self.dependency_map.clone().keys().cloned().collect();
        let diff: BTreeSet<_> = found_packages.difference(&packages).collect();

        dbg!(diff);

        self.dependency_map.clone()
    }

    pub fn dependencies(&mut self, package: &Package) -> BTreeSet<Package> {
        if self.dependency_map.contains_key(&package) {
            return BTreeSet::new();
        }

        // Make sure that there's something for no infinite loops.
        self.dependency_map.insert(package.clone(), BTreeSet::new());
        let mut package_deps = self.manager.dependency_query(&package);

        for d in package_deps.clone() {
            let extension = match self.dependency_map.get(&d) {
                Some(p) => p.clone(),
                None => self.dependencies(&d),
            };

            package_deps.extend(extension);
        }

        self.dependency_map
            .insert(package.clone(), package_deps.clone());

        package_deps
    }
}
