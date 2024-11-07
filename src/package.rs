use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::run_command;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Package {
    pub name: String,
}

impl Package {
    pub fn fetch_dependencies(&self) -> HashSet<Self> {
        let mut mem: HashMap<Self, HashSet<Self>> = HashMap::new();

        self.fetch_dependencies_mem(&mut mem).clone()
    }

    pub fn fetch_amount(&self) -> HashMap<Self, usize> {
        let mut map: HashMap<Self, usize> = HashMap::new();
        self.fetch_amount_mem(&mut map);
        map
    }

    fn exec_dependency_search(&self) -> HashSet<Self> {
        let search = run_command(format!("pacman -Qi {}", &self.name));

        search
            .split('\n')
            .filter(|s| s.contains("Depends On")) // Find the line that contains dependencies
            .map(|s| s.split(':').nth(1).unwrap()) // Remove the "Depends On :" Section
            .filter(|s| !s.contains("None")) // If there's no dependencies, remove the line
            .flat_map(|s| s.split(' ').collect::<Vec<&str>>())
            .filter(|s| !s.is_empty()) // Split up to package names
            .filter_map(|p| match Package::from_str(p) {
                Ok(p) => Some(p),
                _ => None,
            }) // extract as packages
            .collect()
    }

    fn fetch_amount_mem<'a>(
        &'a self,
        found: &'a mut HashMap<Self, usize>,
    ) -> &'a mut HashMap<Self, usize> {
        if found.contains_key(self) {
            return found;
        }

        let dependencies = self.exec_dependency_search();

        let amount: usize = found
            .iter()
            .filter(|(k, _)| dependencies.contains(k))
            .map(|(_, v)| v)
            .sum();

        found.insert(self.clone(), amount + dependencies.len());
        found // might want to remove and put something else
    }

    fn fetch_dependencies_mem<'a>(
        &'a self,
        found: &'a mut HashMap<Self, HashSet<Self>>,
    ) -> HashSet<Self> {
        if found.contains_key(&self.clone()) {
            return HashSet::new();
        }

        let dependencies = self.exec_dependency_search();
        found.insert(self.clone(), dependencies.clone());

        let _ = dependencies
            .iter()
            .flat_map(|p| p.fetch_dependencies_mem(found));

        let mut values: HashSet<Self> = found.values().flatten().cloned().collect();
        let keys: HashSet<Self> = found.keys().cloned().collect();
        values.extend(keys);

        values
    }
}

impl FromStr for Package {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();

        match split.len() {
            1 | 2 => Ok(Package {
                name: split[0].clone(),
                ..Default::default()
            }),
            _ => Err(()),
        }
    }
}
