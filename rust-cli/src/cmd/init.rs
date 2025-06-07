use anyhow::{Ok,Result};
use clap::Args;
use crate::utils::{self, config::Builder}; 


#[derive(Debug,Args)]
#[command(version,about,long_about = None)]
pub struct InitArgs {
    #[arg(default_value = "rust-cli",short = 'n' , help = "the name of the application")]
    app_name:  String,
    #[arg(default_value = "surya/rust-cli" ,short = 'p', help = "default app config file")]
   default_path: String,
   #[arg(short = 's' , help = "db connection string")]
   pub db_conn_str: String,
}

pub fn main(args: &InitArgs, cfg: &mut utils::config::AppConfig) -> Result<()> {
    if args.db_conn_str == ""{
       println!("Please provide the connection string");
       return Ok(())
    }
    let mut config = utils::config::AppConfig::new();
    config
          .set_config_path(args.default_path.clone())
          .set_db_conn_str(&args.db_conn_str)
          .Build();
    cfg.update(&config);

    Ok(())
}