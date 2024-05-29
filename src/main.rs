use std::error::Error;

use clap::Parser;

use cloudflare::endpoints::dns::DnsContent;
use cmd::Cli;
use ddns::{content_to_string, get_records, update_record, Params};
use ip::get_ipv4;

mod cmd;
mod ddns;
mod ip;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let configs = cli.get_config()?;

    let ipv4 = get_ipv4(None).await?;
    println!("Updating A records of '{}' to {ipv4}\n", cli.name);

    let records = get_records(configs.clone(), &cli.name).await?;

    if records.len() < 1 {
        panic!("No records found for name {:?}", cli.name);
    }

    for record in records.iter() {
        println!(
            "Updating {} = {}...",
            record.name,
            content_to_string(&record.content)
        );

        if let DnsContent::AAAA { .. } = record.content {
            println!("  WARN: AAAA records are currently not supported. Skipping.");
            continue;
        }

        let params = Params {
            id: record.id.clone(),
            name: cli.name.clone(),
        };
        update_record(&configs, &params, ipv4.into()).await?;
    }

    println!();
    println!("-----");
    println!("Done!");
    Ok(())
}
