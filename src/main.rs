use tera::Tera;

// rusty Main function
mod meta;
mod page;
mod template;

use template::Template;

pub fn main() {
  println!("Rusty");
  let tpl = Template::load_template("_tpl/**/*.html");
  println!("{:?}", tpl.templates);
}
