// Template handling
//

use tera::{Tera, Context};

use crate::page::Page;

pub struct Template {
    pub templates: Tera,
}

impl Template {
    pub fn load_template(path: &str) -> Self {
        // use globbing
        let mut tera = match Tera::new(path) {
            Ok(a) => a,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        // turn off autoescape
        tera.autoescape_on(vec![]);

        // tera.register_filter("test", test_filter);

        Template {
            templates: tera,
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
    pub fn render(&self, tpl: &str, page: &mut Page) {
        // Convert Metadata into Context
        let mut ctx: Context = page.get_context();
        ctx.insert("body", &page.html);
        // Add html into body
        let rendered = self.templates.render(tpl, &ctx).unwrap();

        // Store back to page
        page.html = rendered.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_new() {
        let tpl = Template::load_template("tests/_tpl/*.html");
        let mut ctx = Context::new();
        ctx.insert("title", "Test");
        ctx.insert("body", "Test Body");
        let rendered = tpl.templates.render("default.html", &ctx)
                                    .unwrap();
        println!("{}", rendered);
    }
}