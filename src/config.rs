use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct AppConfig {
    #[arg(long, default_value = "0.0.0.0", env = "APP_IPV4")]
    pub(crate) ipv4: String,
    #[arg(long, default_value = "1000", env = "APP_PORT")]
    pub(crate) port: u16,
    #[arg(
        long,
        default_value_os_t = std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        env = "APP_DATA_DIR"
    )]
    pub(crate) data_dir: PathBuf,
}


impl Default for AppConfig{
    fn default() -> Self {
        Self::parse()
    }
}