// Page struct
//
// Page is consisted of Meta and Text
use crate::meta::Meta;

pub struct Page {
    meta: Meta,
    md: String,  // TODO: Markdown
    text: String
}

impl Page {

    // load content from file (.md) and store into md
    pub fn load_content(&self) {

    }
}
