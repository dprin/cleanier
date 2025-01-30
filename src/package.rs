use std::{marker::PhantomData, str::FromStr};

use crate::manager::Manager;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Package<M> {
    pub name: String,
    t: PhantomData<M>,
}

impl<M> Package<M> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            t: PhantomData::default(),
        }
    }
}

impl<M: Manager> FromStr for Package<M> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        M::parse_package(s)
    }
}
