use anyhow::{Ok,Result};
use clap::{Command, Parser, Subcommand};

mod cmd;
mod utils;
#[derive(Parser)]
#[command(version,about,long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   Init(cmd::init::InitArgs),
   Update(cmd::update::UpdateArgs),

   Configure(cmd::config::ConfigArgs),

}
fn main() -> Result<()> {
   let mut confg: utils::config::AppConfig = confy::load("rust_cli",None)?;
   println!("full path {:?}",confg);
   let full_path = confy::get_configuration_file_path("rust_cli", None);
   println!("full path: {:?}",full_path);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(args) => {
            cmd::init::main(args, &mut confg)?;
        },
        Commands::Update(args) => {
            cmd::update::main(args, &mut confg)?;
        },
        Commands::Configure(args) => {
            cmd::config::main(args , &mut confg)?;
        }
    }
   Ok(())
}
