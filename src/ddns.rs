use std::{error::Error, fs::File, net::IpAddr};

use cloudflare::{
    endpoints::dns::{
        DnsContent, DnsRecord, ListDnsRecords, ListDnsRecordsParams, UpdateDnsRecord,
        UpdateDnsRecordParams,
    },
    framework::{
        async_api::Client, auth::Credentials, response::ApiResponse, Environment,
        HttpApiClientConfig,
    },
};
use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateConfigs {
    pub token: String,
    pub key: String,
    pub zone_identifier: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
    pub custom_url: Option<Url>,
}

#[derive(Clone, Debug)]
pub struct Params {
    pub id: String,
    pub name: String,
}

impl UpdateConfigs {
    pub fn read_yaml(file: &str) -> Result<Self, Box<dyn Error>> {
        let f = File::open(file)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}

pub async fn get_records(
    configs: UpdateConfigs,
    name: &str,
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

    let get_params = ListDnsRecordsParams {
        name: Some(name.to_owned()),
        ..Default::default()
    };

    let endpoint = ListDnsRecords {
        zone_identifier: &configs.zone_identifier,
        params: get_params,
    };

    let resp = client.request(&endpoint).await?;

    Ok(resp.result)
}

pub async fn update_record(
    configs: &UpdateConfigs,
    params: &Params,
    addr: IpAddr,
) -> Result<DnsRecord, Box<dyn Error>> {
    // let credentials = Credentials::UserAuthKey {
    //     email: "kino.ma.ms@gmail.com".to_owned(),
    //     key: configs.key.clone(),
    // };
    let credentials = Credentials::UserAuthToken {
        token: configs.token.clone(),
    };
    let client_config = HttpApiClientConfig::default();

    let environment = match &configs.custom_url {
        Some(u) => Environment::Custom(u.clone()),
        None => Environment::Production,
    };

    let client = Client::new(credentials, client_config, environment)?;

    let content = match addr {
        IpAddr::V4(ipv4) => DnsContent::A { content: ipv4 },
        IpAddr::V6(ipv6) => DnsContent::AAAA { content: ipv6 },
    };
    let resp = send_request(&client, &configs, params, content).await?;

    Ok(resp.result)
}

async fn send_request(
    client: &Client,
    configs: &UpdateConfigs,
    params: &Params,
    content: DnsContent,
) -> ApiResponse<DnsRecord> {
    let update_params = UpdateDnsRecordParams {
        ttl: configs.ttl,
        proxied: configs.proxied,
        name: &params.name,
        content,
    };

    let endpoint = UpdateDnsRecord {
        zone_identifier: &configs.zone_identifier,
        identifier: &params.id,
        params: update_params,
    };

    client.request(&endpoint).await
}

pub fn content_to_string(content: &DnsContent) -> String {
    use DnsContent::*;

    match content {
        A { content } => content.to_string(),
        AAAA { content } => content.to_string(),
        TXT { content } => format!("TXT '{content}'"),
        MX { content, priority } => format!("MX {content} ({priority})"),
        CNAME { content } => format!("CNAME => {content}"),
        SRV { content } => format!("SRV {content}"),
        NS { content } => format!("NS {content}"),
    }
}
