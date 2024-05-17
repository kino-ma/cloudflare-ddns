use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 't', long = "token")]
    token: String,

    #[arg(short = 'k', long = "key")]
    key: String,
}
