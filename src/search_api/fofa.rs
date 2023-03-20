use base64::{engine::general_purpose, Engine as _};
use log::{info, error};
use reqwest;
use serde_json;
use std::{collections::HashMap, time::Duration};

async fn query_fofa(
    query: &str,
    size: i32,
    email: &str,
    token: &str,
    search_proxy: &str,
) -> Result<serde_json::Value, bool> {
    let fofa_api_url = String::from("https://fofa.info/api/v1/search/all");

    let mut params = HashMap::new();
    params.insert("email", email);
    params.insert("key", token);
    let q_size = &size.to_string()[..];
    params.insert("size", q_size);
    params.insert("fields", "host");
    let qbase64 = general_purpose::URL_SAFE.encode(query);
    params.insert("qbase64", &qbase64[..]);

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
        .get(fofa_api_url)
        .query(&params)
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

pub async fn get_socks5_proxy_fofa(
    query: &str,
    size: i32,
    email: &str,
    token: &str,
    search_proxy: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    info!("{}", "Searching from fofa...");
    let mut result: Vec<String> = Vec::new();
    match query_fofa(query, size, email, token, search_proxy).await {
        Ok(data) => {
            for i in data["results"].as_array().unwrap().iter() {
                let ip_port = &i.to_string()[..];
                let len = ip_port.len();
                result.push(ip_port[1..len - 1].to_string());
            }
        },
        Err(_) => {
            return Ok(result);
        }
    }
    info!(" - Get {} proxy from {}", result.len(), "fofa");
    Ok(result)
}
