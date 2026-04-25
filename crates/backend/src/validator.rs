use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::error::ApiError;
use reqwest::Url;

pub fn validate_url(input: &str) -> Result<(), ApiError> {
    if input.trim().is_empty() {
        return Err(ApiError::BadRequest("URL is required.".to_string()));
    }

    if !(input.starts_with("http://") || input.starts_with("https://")) {
        return Err(ApiError::BadRequest(
            "URL must start with http:// or https://".to_string(),
        ));
    }

    Ok(())
}

pub fn parse_url(input: &str) -> Result<Url, ApiError> {
    validate_url(input)?;
    let url = Url::parse(input)
        .map_err(|error| ApiError::BadRequest(format!("invalid URL format: {error}")))?;

    if url.host().is_none() {
        return Err(ApiError::BadRequest("URL host is required.".to_string()));
    }

    Ok(url)
}

fn is_forbidden_ipv4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    let is_cgnat = octets[0] == 100 && (64..=127).contains(&octets[1]);
    let is_benchmark_net = octets[0] == 198 && (18..=19).contains(&octets[1]);
    let is_future_reserved = octets[0] >= 240;
    let is_zero_net = octets[0] == 0;

    ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_documentation()
        || ip.is_unspecified()
        || ip.is_multicast()
        || is_cgnat
        || is_benchmark_net
        || is_future_reserved
        || is_zero_net
}

fn is_forbidden_ipv6(ip: Ipv6Addr) -> bool {
    if let Some(v4_mapped) = ip.to_ipv4() {
        return is_forbidden_ipv4(v4_mapped);
    }

    let segments = ip.segments();
    let first = segments[0];
    let second = segments[1];
    let is_unique_local = (first & 0xfe00) == 0xfc00;
    let is_link_local = (first & 0xffc0) == 0xfe80;
    let is_documentation = first == 0x2001 && second == 0x0db8;

    ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_multicast()
        || is_unique_local
        || is_link_local
        || is_documentation
}

pub fn is_forbidden_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_forbidden_ipv4(v4),
        IpAddr::V6(v6) => is_forbidden_ipv6(v6),
    }
}

pub async fn validate_target_url(url: &Url) -> Result<(), ApiError> {
    let host = url
        .host_str()
        .ok_or_else(|| ApiError::BadRequest("URL host is required.".to_string()))?;

    let host_lc = host.to_ascii_lowercase();
    if host_lc == "localhost" || host_lc.ends_with(".localhost") {
        return Err(ApiError::BadRequest(
            "localhost targets are not allowed for scraping.".to_string(),
        ));
    }

    let port = url
        .port_or_known_default()
        .ok_or_else(|| ApiError::BadRequest("URL port is invalid.".to_string()))?;

    let addrs = tokio::net::lookup_host((host, port))
        .await
        .map_err(|error| ApiError::BadRequest(format!("could not resolve URL host: {error}")))?;

    for addr in addrs {
        if is_forbidden_ip(addr.ip()) {
            return Err(ApiError::BadRequest(
                "target resolves to a private or non-routable address, which is not allowed."
                    .to_string(),
            ));
        }
    }

    Ok(())
}