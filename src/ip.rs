use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use reqwest::Client;

const CHECK_IP_URL: &'static str = "https://checkip.amazonaws.com";

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
        .map_err(|e| format!("ip::get: request failed: {e:?}"))?
        .text()
        .await
        .map_err(|e| format!("ip::get: receiving response text failed: {e}"))?;

    let trimed = addr_str.trim();
    trimed
        .parse()
        .map_err(|_| format!("failed to parse returned IP address: '{trimed}'"))
}

#[cfg(test)]
mod test {
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_ipv4_with_newline() {
        let addr = "133.27.170.188\n";
        addr.parse::<Ipv4Addr>().unwrap_err();

        let parsed: Ipv4Addr = addr.trim().parse().unwrap();
        assert_eq!(parsed, Ipv4Addr::new(192, 0, 2, 1));
    }

    #[test]
    fn test_parse_ipv4() {
        let addr = "192.0.2.1";
        let parsed: Ipv4Addr = addr.parse().unwrap();
        assert_eq!(parsed, Ipv4Addr::new(192, 0, 2, 1));
    }
}
