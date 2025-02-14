pub mod api;
pub mod config;
pub mod directory;
pub mod file;
pub mod http;
pub mod resolver;
pub mod path;

use clap::Parser;
use crate::config::Config;
use crate::http::HTTP;
use log::error;
use std::env;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "FS API")]
struct Args {
    #[arg(long)]
    api_host: Option<String>,

    #[arg(long)]
    api_port: Option<u16>,

    #[arg(short, long, default_value = "./config.yaml")]
    config: String,

    #[arg(long)]
    log_level: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::from_yaml(&args.config).await.unwrap();
    let log_level = args.log_level
        .or_else(|| config.api.as_ref()?.log.as_ref()?.level.clone())
        .or_else(|| env::var("LOG_LEVEL").ok())
        .unwrap_or_else(|| "info".to_string());
    env::set_var("RUST_LOG", log_level);
    env_logger::init();
    let api_host = args.api_host
        .or_else(|| config.api.as_ref()?.listen.as_ref()?.host.clone())
        .unwrap_or_else(|| {
            error!("Error: API host must be set via --api-host or config file");
            std::process::exit(1);
        });
    let api_port = args.api_port
        .or_else(|| config.api.as_ref()?.listen.as_ref()?.port.clone())
        .unwrap_or_else(|| {
            error!("Error: API port must be set via --api-port or config file");
            std::process::exit(1);
        });
    let listen = (api_host.parse::<std::net::IpAddr>().unwrap(), api_port).into();
    HTTP::server(listen, config).await;
}
