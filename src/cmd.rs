use clap::Parser;

use crate::ddns::{Params, UpdateConfigs};

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub name: String,
    pub content: String,

    #[arg(short, long)]
    pub file: Option<String>,

    #[arg(short, long)]
    pub token: Option<String>,
    #[arg(short, long)]
    pub key: Option<String>,
    #[arg(short = 'i', long)]
    pub zone_identifier: Option<String>,
    #[arg(long)]
    pub ttl: Option<String>,
    #[arg(long)]
    pub proxied: Option<String>,
}

impl Cli {
    pub fn get_params(&self) -> Params {
        Params {
            name: self.name.clone(),
            content: self.content.clone(),
        }
    }

    pub fn get_config(&self) -> Result<UpdateConfigs, String> {
        match self.file.as_deref() {
            Some(file) => Ok(UpdateConfigs::read_yaml(file).map_err(|e| e.to_string())?),
            None => self.clone().try_into().map_err(|e| {
                format!("either '-f' option or all command line parameters must be valid: {e}")
            }),
        }
    }

    pub fn try_into_configs(self) -> Option<UpdateConfigs> {
        let configs = UpdateConfigs {
            token: self.token?,
            key: self.key?,
            zone_identifier: self.zone_identifier?,
            ttl: self.ttl,
            proxied: self.proxied,
        };

        Some(configs)
    }
}

impl TryInto<UpdateConfigs> for Cli {
    type Error = String;

    fn try_into(self) -> Result<UpdateConfigs, Self::Error> {
        self.try_into_configs()
            .ok_or("Cli::try_into: missing some field".to_owned())
    }
}
