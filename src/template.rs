// Template handling
//

use tera::{Tera, Context};

use crate::page::Page;

pub struct Template {
    pub templates: Tera,
    pub context: Context,
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
            context: Context::new(),
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
    pub fn render(&self, tpl: &str, page: &Page) -> String {
        // Convert Metadata into Context
        let mut ctx: Context = page.get_context();
        ctx.insert("body", &page.html);
        // Add html into body
        self.templates.render(tpl, &ctx).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_new() {
        let mut tpl = Template::load_template("tests/_tpl/*.html");
        tpl.context.insert("title", "Test");
        let rendered = tpl.templates.render("default.html", &tpl.context).unwrap();
        println!("{}", rendered);
    }
}