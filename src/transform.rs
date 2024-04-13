// Transform functions

use std::path::{PathBuf, Components};

// Libraries for Path manipulation
#[allow(dead_code)]
pub fn transform_none(path: &PathBuf) -> PathBuf {
    path.clone()
}

// Remove top N directory to the destination uri.
//
// Args:
//   depth: the number of directory depth to remove.
//
// Returns:
//   Transformed the path.
#[allow(unused)]
pub fn transform_moveup(depth: usize, path: &PathBuf) -> PathBuf {
    if depth == 0 {
        return path.clone();
    }

    let mut components = path.components();
    let mut depth_adjusted = depth;
    if ! path.starts_with("/") {
        depth_adjusted = depth_adjusted - 1;
    }
    if path.starts_with("../") {
        println!("move up transform does not support relative ../");
        panic!();
    }
    components.nth(depth_adjusted);
    components.as_path().to_path_buf()
}

// TODO: Transform each markdown into a separate directory.
// e.g) review/item1.md --> review/item1/index.html
#[allow(unused)]
pub fn transform_seo(path: &PathBuf) -> PathBuf {
    // TODO: Implement.
    path.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_moveup_2() {
        let path = PathBuf::from("tests/page/review/item1.md");
        let new_path = transform_moveup(2, &path);

        assert_eq!(new_path, PathBuf::from("review/item1.md"));
    }

    #[test]
    fn transform_moveup_3_abs() {
        let path = PathBuf::from("/media/page/review/item.png");
        let new_path = transform_moveup(2, &path);

        assert_eq!(new_path, PathBuf::from("review/item.png"));
    }

    #[test]
    #[should_panic]
    fn transform_moveup_2_upper() {
        let path = PathBuf::from("../../review/item.md");
        let _ = transform_moveup(2, &path);
    }
}