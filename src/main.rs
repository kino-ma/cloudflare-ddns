use std::error::Error;

use clap::Parser;
use cmd::Cli;

mod cmd;
mod ddns;
mod ip;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let configs = cli.get_config()?;
    let params = cli.get_params();

    println!("configs = {configs:?}");
    println!("params  = {params:?}");

    Ok(())
}
