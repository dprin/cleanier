#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Package {
    pub name: String,
}

impl Package {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

pub trait Parseable {
    fn parse_package(s: &str) -> Result<Package, ()>;
}

#[cfg(test)]
mod tests {
    use super::Package;

    #[test]
    fn eq() {
        let a = Package::new("something");
        let b = Package::new("something");

        assert!(a == b);
    }
}
