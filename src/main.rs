use std::error::Error;

use clap::Parser;

use cmd::Cli;
use ip::{get_ipv4, get_ipv6};

mod cmd;
mod ddns;
mod ip;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let configs = cli.get_config()?;
    let params = cli.get_params();

    println!("configs = {configs:?}");
    println!("params  = {params:?}");

    let ipv4 = get_ipv4(None).await?;
    println!("IPv4 = {ipv4}");

    // let ipv6 = get_ipv6(None).await?;
    // println!("IPv6 = {ipv6}");

    Ok(())
}
