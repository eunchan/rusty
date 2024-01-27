// Assets
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Asset {
    pub path: PathBuf, // file
    pub uri: PathBuf,  // URI (without fileserver cfg)
    pub hash: u64,     // Hash output
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

impl Asset {
    pub fn new(path: PathBuf) -> Self {
        let mut s = DefaultHasher::new();
        path.hash(&mut s);
        Asset {
            path: path,
            uri: PathBuf::from(""),
            hash: s.finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        let asset1 = Asset::new(PathBuf::from("tests/img1.jpg"));
        let mut asset2 = Asset::new(PathBuf::from("tests/img1.jpg"));
        asset2.uri = PathBuf::from("/media/img1.jpg");
        let mut asset3 = Asset::new(PathBuf::from("tests/img2.jpg"));
        asset3.uri = PathBuf::from("/media/img2.jpg");

        assert_eq!(asset1.hash, asset2.hash);
        assert_ne!(asset2.hash, asset3.hash);
    }
}
