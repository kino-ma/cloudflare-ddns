use std::{
    error::Error,
    fs::File,
    net::{Ipv4Addr, Ipv6Addr},
};

use cloudflare::{
    endpoints::dns::{DnsContent, DnsRecord, UpdateDnsRecord, UpdateDnsRecordParams},
    framework::{
        async_api::Client, auth::Credentials, response::ApiResponse, Environment,
        HttpApiClientConfig,
    },
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct UpdateConfigs {
    pub token: String,
    pub key: String,
    pub zone_identifier: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
    pub custom_url: Option<Url>,
}

#[derive(Debug)]
pub struct Params {
    pub name: String,
}

impl UpdateConfigs {
    pub fn read_yaml(file: &str) -> Result<Self, Box<dyn Error>> {
        let f = File::open(file)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

pub async fn update_record(
    params: Params,
    configs: UpdateConfigs,
    ipv4: Ipv4Addr,
    ipv6: Option<Ipv6Addr>,
) -> Result<Vec<DnsRecord>, Box<dyn Error>> {
    let credentials = Credentials::UserAuthToken {
        token: configs.token.clone(),
    };
    let client_config = HttpApiClientConfig::default();

    let environment = match &configs.custom_url {
        Some(u) => Environment::Custom(u.clone()),
        None => Environment::Production,
    };

    let client = Client::new(credentials, client_config, environment)?;

    let content_a = DnsContent::A { content: ipv4 };
    let resp_a = send_request(&client, &configs, &params.name, content_a).await?;

    let mut records = vec![resp_a.result];

    if let Some(ipv6) = ipv6 {
        let content_aaaa = DnsContent::AAAA { content: ipv6 };
        let resp_aaaa = send_request(&client, &configs, &params.name, content_aaaa).await?;
        records.push(resp_aaaa.result);
    }

    Ok(records)
}

async fn send_request(
    client: &Client,
    configs: &UpdateConfigs,
    name: &str,
    content: DnsContent,
) -> ApiResponse<DnsRecord> {
    let update_params = UpdateDnsRecordParams {
        ttl: configs.ttl,
        proxied: configs.proxied,
        name,
        content,
    };

    let endpoint = UpdateDnsRecord {
        zone_identifier: &configs.zone_identifier,
        identifier: &configs.zone_identifier,
        params: update_params,
    };

    client.request(&endpoint).await
}
