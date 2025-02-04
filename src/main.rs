use anyhow::{Ok,Result};
use clap::{Command, Parser, Subcommand};

mod cmd;

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
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(args) => {
            cmd::init::main(args)?;
        },
        Commands::Update(args) => {
            cmd::update::main(args)?;
        },
        Commands::Configure(args) => {
            cmd::config::main(args)?;
        }
    }
   Ok(())
}
