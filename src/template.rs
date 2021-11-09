// Template handling
//

use tera::Tera;

use crate::page::Page;

pub struct Template {
    pub templates: Tera
}

impl Template {
    pub fn load_template(path: &str) -> Template {
        // use globbing
        let mut tera = match Tera::new(path) {
            Ok(a) => a,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        // tera.register_filter("test", test_filter);

        Template {
            templates: tera
        }
    }

    // Render with the Page & Meta
    //
    // - Find appropriate template in page.meta
    // - Build tera::Context from page
    // - Render
    //
    // Assumptions:
    // - page.text is already converted into html from markdown
    // - page.text relative URLs are replaced to the direct path
    pub fn render(&self, page: &Page) -> bool {
        false
    }
}
