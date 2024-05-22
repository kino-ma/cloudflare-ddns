use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub name: String,
    pub content: String,

    #[arg(short, long)]
    pub token: String,

    #[arg(short, long)]
    pub key: String,

    #[arg(short = 'i', long)]
    pub zone_identifier: String,

    #[arg(long)]
    pub ttl: Option<String>,

    #[arg(long)]
    pub proxied: Option<String>,
}
