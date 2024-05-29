use std::error::Error;

use clap::Parser;

use cmd::Cli;
use ddns::{get_records, update_record, Params};
use ip::get_ipv4;

mod cmd;
mod ddns;
mod ip;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let configs = cli.get_config()?;

    println!("configs = {configs:?}");

    let ipv4 = get_ipv4(None).await?;
    println!("Updating address to: {ipv4}");

    let records = get_records(configs.clone(), &cli.name).await?;
    println!("Got records: {records:?}");

    if records.len() < 1 {
        panic!("No records found for name {:?}", cli.name);
    }

    for record in records.iter() {
        let params = Params {
            id: record.id.clone(),
            name: cli.name.clone(),
        };
        let resp = update_record(&configs, &params, ipv4.into()).await?;
        println!("Updated: {resp:?}");
    }

    Ok(())
}
