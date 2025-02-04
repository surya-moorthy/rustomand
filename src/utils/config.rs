use serde::{Serialize,Deserialize};
use std::path::PathBuf;

#[derive(Debug,Serialize,Deserialize)]
pub struct AppConfig {
   app_name:  String,
   default_path: PathBuf,
   pub db_conn_str: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let app_name = env!("CARGO_PKG_NAME");
        Self {
            app_name: app_name.to_string(),
            default_path: "rust-cli".into(),
            db_conn_str: String::new()
        }
    }
}