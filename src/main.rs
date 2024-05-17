use clap::Parser;
use cmd::Cli;

mod cmd;

fn main() {
    let cli = Cli::parse();

    let token = cli.token;
    let key = cli.key;
    let identifier = cli.zone_identifier;
}
