use std::{fs::File, path::PathBuf};

use clap::Parser;
use log::debug;
use tera::Tera;

mod args;
mod ctx;

mod wakatime;

#[derive(Debug)]
pub enum ReadmeToolError {
    TemplateFileInvalid(PathBuf),
    FailedToAddTemplateFiles(Vec<PathBuf>)
}

fn main() -> Result<(), ReadmeToolError> {
    
    debug!("Parsing command line args...");

    let args = args::CmdlineArgs::parse();

    debug!("{:?}", args);

    let mut tera = Tera::new();

    load_fp_into_tera(&mut tera, args.template_path)?;

    return Ok(());
}

// Attempts to load a single template into the provided Tera instance.
fn load_fp_into_tera(tera: &mut Tera, fp: PathBuf) -> Result<(), ReadmeToolError> {

    debug!("Loading template: {}", fp.display());

    // Checks to make sure the template file exists and ends with .md
    if fp.exists() && fp.ends_with(".md") {
        return tera.add_template_file(fp.clone(), Some("root"))
            .or(Err(ReadmeToolError::FailedToAddTemplateFiles(vec![fp])));
    } 

    // If the template file does not exist, or contains an .md, then return that the template
    // path is invalid.
    return Err(ReadmeToolError::TemplateFileInvalid(fp));

}