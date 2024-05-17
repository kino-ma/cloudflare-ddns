use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 't', long = "token")]
    pub token: String,

    #[arg(short = 'k', long = "key")]
    pub key: String,

    #[arg(short = 'i', long = "zone-identifier")]
    pub zone_identifier: String,
}
