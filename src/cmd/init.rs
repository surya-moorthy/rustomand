use anyhow::{Ok,Result};
use clap::Args;

#[derive(Debug,Args)]
#[command(version,about,long_about = None)]
pub struct InitArgs {}

pub fn main(args: &InitArgs) -> Result<()> {
    Ok(())
}