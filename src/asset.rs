// Assets
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Asset {
    pub path: PathBuf, // file
    pub uri: PathBuf,  // URI (without fileserver cfg)
}

impl Hash for Asset {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Asset:\n  path: {}\n  uri: {}",
            self.path.to_string_lossy(), self.uri.to_string_lossy())
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Asset {
    pub fn new(path: PathBuf) -> Self {
        Asset {
            path: path,
            uri: PathBuf::from(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        let asset1 = Asset {
            path: PathBuf::from("tests/img1.jpg"),
            uri: PathBuf::from(""),
        };
        let asset2 = Asset {
            path: PathBuf::from("tests/img1.jpg"),
            uri: PathBuf::from("/media/img1.jpg"),
        };
        let asset3 = Asset {
            path: PathBuf::from("tests/img2.jpg"),
            uri: PathBuf::from("/media/img2.jpg"),
        };

        assert_eq!(calculate_hash(&asset1), calculate_hash(&asset2));
        assert_ne!(calculate_hash(&asset2), calculate_hash(&asset3));
    }
}
