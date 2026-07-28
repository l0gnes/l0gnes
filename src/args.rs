use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CmdlineArgs {

    #[arg(short, long)]
    pub template_path: PathBuf,

}