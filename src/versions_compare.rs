use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    components: Vec<u32>,
}

impl Version {
    pub fn new(version_str: &str) -> Self {
        let components = version_str
            .split('.')
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .collect();
        Version { components }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let len = self.components.len().max(other.components.len());

        for i in 0..len {
            let self_comp = *self.components.get(i).unwrap_or(&0);
            let other_comp = *other.components.get(i).unwrap_or(&0);

            match self_comp.cmp(&other_comp) {
                Ordering::Equal => continue,
                other => return Some(other),
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