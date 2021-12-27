use clap::Parser;
use std::{env, net::IpAddr};

#[derive(Debug, Parser)]
pub struct ServerConfig {
    #[clap(default_value = "127.0.0.1", env)]
    pub host: IpAddr,

    #[clap(default_value = "5003", env)]
    pub port: u16,
}
