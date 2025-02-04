use anyhow::{Ok,Result};
use clap::Args;

#[derive(Debug,Args)]
#[command(version,about,long_about = None)]
pub struct ConfigArgs {}

pub fn main(args: &ConfigArgs) -> Result<()> {
    Ok(())
}