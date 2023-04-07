// Assets
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

pub struct Asset {
    pub path: PathBuf, // file
    pub uri: PathBuf,  // URI (without fileserver cfg)
}

impl Hash for Asset {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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
