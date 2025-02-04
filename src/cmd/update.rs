use anyhow::{Ok,Result};
use clap::Args;

#[derive(Debug,Args)]
#[command(version,about,long_about = None)]
pub struct UpdateArgs {}

pub fn main(args: &UpdateArgs) -> Result<()> {
    Ok(())
}