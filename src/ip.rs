use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use reqwest::Client;

const CHECK_IP_URL: &'static str = "https://domains.google.com/checkip";

pub async fn get_ipv4(preferred_addr: Option<Ipv4Addr>) -> Result<Ipv4Addr, String> {
    let addr: IpAddr = preferred_addr.unwrap_or("0.0.0.0".parse().unwrap()).into();
    let client = Client::builder()
        .local_address(addr)
        .build()
        .map_err(|e| format!("failed to create client: {e}"))?;

    get(client).await
}

pub async fn get_ipv6(preferred_addr: Option<Ipv6Addr>) -> Result<Ipv6Addr, String> {
    let addr: IpAddr = preferred_addr.unwrap_or("::".parse().unwrap()).into();
    let client = Client::builder()
        .local_address(addr)
        .build()
        .map_err(|e| format!("failed to create client: {e}"))?;

    get(client).await
}

async fn get<T: FromStr>(client: Client) -> Result<T, String> {
    let addr_str: String = client
        .get(CHECK_IP_URL)
        .send()
        .await
        .map_err(|e| format!("ip::get: request failed: {e}"))?
        .text()
        .await
        .map_err(|e| format!("ip::get: receiving response text failed: {e}"))?;

    addr_str
        .parse()
        .map_err(|_| format!("failed to parse returned IP address"))
}
