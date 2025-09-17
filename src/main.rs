use std::fs;
use tera::{Context, Tera};

pub mod context;

const TEMPLATE_MD_PATH: &'static str = "./template-readme.md";
const OUT_PATH: &'static str = "./readme.md";

fn main() {
    // Load any evironment variables we may need
    dotenvy::dotenv().ok();

    // Check to make sure that the file exists!
    if !fs::exists(TEMPLATE_MD_PATH).unwrap() {
        panic!("Failed to find template-readme.md!");
    }

    // We already know this is probably going to return the file data so I'm unwrapping it lol
    let template_md = fs::read_to_string(TEMPLATE_MD_PATH).unwrap();

    // Create the tera struct so we can start dealing with filling the funny values in
    let mut tera = Tera::default();

    // Add our template file under the name "readme"
    let _ = tera.add_raw_template("readme", template_md.as_str());

    // Call our funny context function
    let context = context::get_context();

    // Call our funky function registration function... function function function func funck function
    let _ = context::register_funcs(&mut tera);

    // use tera to render
    let rendered = tera.render("readme", &context).unwrap();

    // dump our fresh new readme
    let _ = fs::write(OUT_PATH, rendered);
}
