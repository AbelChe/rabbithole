use std::time::Duration;

use log::{info, error};
use reqwest;
use reqwest::header::HeaderMap;
use serde_json;

async fn query_quake(
    query: &str,
    size: i32,
    token: &str,
    search_proxy: &str,
) -> Result<serde_json::value::Value, bool> {
    let quake_api_url = String::from("https://quake.360.net/api/v3/search/quake_service");

    let q_size = &size.to_string()[..];
    let data = serde_json::json!({
        "query": query,
        "start": "0",
        "size": q_size,
    });

    let mut headers = HeaderMap::new();
    headers.insert("X-QuakeToken", token.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = if search_proxy.is_empty() {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap()
    } else {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .proxy(reqwest::Proxy::all(search_proxy.to_string()).unwrap())
            .build()
            .unwrap()
    };

    let req = client
        .post(quake_api_url)
        .headers(headers)
        .body(data.to_string())
        .send()
        .await;
    match req {
        Ok(r) => {
            match r.json::<serde_json::Value>().await {
                Ok(v) => return Ok(v),
                Err(e) => {
                    error!("  {:?}", e);
                    return Err(false)
                }
            }
        },
        Err(e) => {
            error!("  {:?}", e);
            return Err(false)
        }
    }
}

pub async fn get_socks5_proxy_quake(
    query: &str,
    size: i32,
    token: &str,
    search_proxy: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    info!("{}", "Searching from quake...");
    let mut result: Vec<String> = Vec::new();
    match query_quake(query, size, token, search_proxy).await {
        Ok(data) => {
            for i in data["data"].as_array().unwrap().iter() {
                let mut ip_port = String::from("");
                let ip = i["ip"].to_string();
                let port = i["port"].to_string();
                let iplen = ip.len();
                ip_port += &ip.to_string()[1..iplen - 1];
                ip_port += ":";
                ip_port += &port;
                result.push(ip_port.to_string());
            }
        },
        Err(_) => {
            return Ok(result);
        }
    }
    info!(" - Get {} proxy from {}", result.len(), "quake");
    Ok(result)
}
