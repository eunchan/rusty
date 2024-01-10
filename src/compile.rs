// Compile the Page
//
// Main part of page process.
//
// - Build AST using comrak
// - Change relative path to absolute path.
//   - if media, possibly with domain (based on cfg or mode)
// - Add relative markdown link list (intra w/ slugs)
// - Call comrak to convert markdown to html
use crate::config::Config;
use crate::page::Page;
use crate::template::Template;
use crate::asset::Asset;

use comrak::{Arena, parse_document, format_html, Options};
use comrak::nodes::{AstNode, NodeValue};

// TODO: Add context, config, other manipulators
pub fn compile_page(p: &mut Page) {
    let arena = Arena::new();
    let root = parse_document(&arena, &p.text, &Options::default());

    for node in root.children() {
        // Node manipulation
    }

    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();
    p.html = String::from_utf8(html).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_compile() {
        let mut item = Page::new(String::from("tests/page.md"));

        compile_page(&mut item);
        assert_eq!(
           item.html,
            "<h2>Section</h2>\n\
            <p>This is a test page.</p>\n\
            <p>It has a <a href=\"https://testpage\">link</a> to external. It also has an [[internal_link]].\n\
            The asset is given in:</p>\n\
            <p><img src=\"image.jpg\" alt=\"\" /></p>\n\
            <h3>Subsection</h3>\n\
            <p>The subsection is heading 3.</p>\n"
        );
    }

    #[test]
    fn template_with_page() {
        let mut item = Page::new(String::from("tests/page.md"));
        let mut tpl = Template::load_template("tests/_tpl/*.html");

        compile_page(&mut item);
        let rendered = tpl.render("default.html", &item);

        println!("{}", rendered);
    }
}