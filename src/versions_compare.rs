use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum VersionComponent {
    Numeric(u32),
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
pub struct Version {
    components: Vec<VersionComponent>,
}

impl Version {
    pub fn new(version_str: &str) -> Self {
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