use crate::package::Package;
use std::collections::{BTreeMap, BTreeSet};

pub mod pacman;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct DependencyCount<M: Manager>(BTreeSet<Package<M>>, usize);

impl<M: Manager> DependencyCount<M> {
    fn add(&mut self, other: &Self) {
        self.1 += other.1;
        self.0.extend(other.0.clone());
    }
}

pub trait Manager: Sized + Clone + Ord {
    fn dependency_query(&self, package: &Package<Self>) -> BTreeSet<Package<Self>>;
    fn get_installed_packages() -> BTreeSet<Package<Self>>;
    fn parse_package(s: &str) -> Result<Package<Self>, ()>;
}

struct Collector<M: Manager> {
    manager: M,
    dependencies: BTreeMap<Package<M>, DependencyCount<M>>,
}

impl<M: Manager> Collector<M> {
    fn new(manager: M) -> Self {
        Self {
            manager,
            dependencies: BTreeMap::new(),
        }
    }

    fn system_dependency_graph(&mut self) -> BTreeMap<Package<M>, DependencyCount<M>> {
        todo!()
    }

    fn get_dependencies(&mut self, package: Package<M>) -> DependencyCount<M> {
        let query = self.manager.dependency_query(&package);
        let mut package_deps = DependencyCount(query.clone(), query.len());

        for d in package_deps.0.clone() {
            let extension = match self.dependencies.get(&d) {
                Some(p) => p.clone(),
                None => self.get_dependencies(d),
            };

            package_deps.add(&extension);
        }

        self.dependencies
            .insert(package.clone(), package_deps.clone());

        package_deps
    }
}
