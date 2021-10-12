use tera::Tera;

// rusty Main function
mod meta;
mod page;
mod template;

pub fn main() {
  println!("Rusty");
  let tpl = template::Template::load_template("_tpl/**/*.html");
  println!("{:?}", tpl.templates);
}