// Page struct
//
// Page is consisted of Meta and Text

use std::path::PathBuf;

use crate::meta::Meta;

pub struct Page {
    pub meta: Meta,
    pub md: Option<PathBuf>,  // TODO: Markdown
    pub uri: PathBuf,
    pub text: String
}

impl Page {
    pub fn new(path: String) -> Self {
        Page {
            meta: Meta::new(),
            md: Some(PathBuf::from(path)),
            uri: PathBuf::new(),
            text: String::from(""),
        }
    }

    // load content from file (.md) and store into md
    pub fn load_content(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_new() {
        let item = Page::new(String::from("page/review/article.md"));
        let mdpath = PathBuf::from("page/review/article.md");
        assert_eq!(item.md.unwrap(), mdpath);
    }
}