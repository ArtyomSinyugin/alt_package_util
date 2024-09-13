use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum VersionComponent {
    Numeric(u64),
    Alphanumeric(String),
}

impl PartialOrd for VersionComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use VersionComponent::*;
        match (self, other) {
            (Numeric(n1), Numeric(n2)) => n1.partial_cmp(n2),
            (Alphanumeric(a1), Alphanumeric(a2)) => a1.partial_cmp(a2),
            (Numeric(_), Alphanumeric(_)) => Some(Ordering::Less),
            (Alphanumeric(_), Numeric(_)) => Some(Ordering::Greater),
        }
    }
}

impl Ord for VersionComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Version {
    components: Vec<VersionComponent>,
}

impl Version {
    fn new(version_str: &str) -> Self {
        let mut components = Vec::new();
        let mut current = String::new();

        for c in version_str.chars() {
            if c.is_digit(10) && current.chars().next().map_or(true, |x| x.is_digit(10)) {
                current.push(c);
            } else {
                if !current.is_empty() {
                    if current.chars().next().unwrap().is_digit(10) {
                        components.push(VersionComponent::Numeric(current.parse().unwrap()));
                    } else {
                        components.push(VersionComponent::Alphanumeric(current.clone()));
                    }
                }
                current.clear();
                current.push(c);
            }
        }

        if !current.is_empty() {
            if current.chars().next().unwrap().is_digit(10) {
                components.push(VersionComponent::Numeric(current.parse().unwrap()));
            } else {
                components.push(VersionComponent::Alphanumeric(current.clone()));
            }
        }

        Version { components }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let len = self.components.len().max(other.components.len());

        for i in 0..len {
            let self_comp = self.components.get(i);
            let other_comp = other.components.get(i);

            match (self_comp, other_comp) {
                (Some(c1), Some(c2)) => match c1.cmp(c2) {
                    Ordering::Equal => continue,
                    other => return Some(other),
                },
                (Some(_), None) => return Some(Ordering::Greater),
                (None, Some(_)) => return Some(Ordering::Less),
                (None, None) => return Some(Ordering::Equal),
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PackageVersionRelease {
    epoch: u32,
    version: Version,
    release: Version,
}

impl PackageVersionRelease {
    pub fn new(epoch: u32, version: &str, release: &str) -> Self {
        let version = Version::new(version);
        let release = if release.len() > 1 {
            let (_, release) = release.split_at(3);
            Version::new(release)
        } else {
            Version::new("")
        };
        PackageVersionRelease { epoch, version, release }
    }
}

impl PartialOrd for PackageVersionRelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.epoch.partial_cmp(&other.epoch) {
            Some(Ordering::Equal) => {
                match self.version.partial_cmp(&other.version) {
                    Some(Ordering::Equal) => self.release.partial_cmp(&other.release),
                    ord => ord,
                }
            },
            ord => ord,
        }
    }
}

impl Ord for PackageVersionRelease {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}