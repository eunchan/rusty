// Template handling
//

use tera::Tera;

struct Template {
};

fn load_template(path: string) -> Template {
    // use globbing
    let tera = match Tera::new("_tpl/**/*.html") {
        Ok(a) => a,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

}
