use log::{info, error};
use reqwest;
use reqwest::header::HeaderMap;
use serde_json;
use std::collections::HashMap;
use std::time::Duration;

// service:"socks5" +after:"2023-02-01" +banner:"Version:5 Method:No Authentication(0x00)" +country:"CN"
async fn query_zoomeye(
    query: &str,
    page_num: i32,
    token: &str,
    search_proxy: &str,
) -> Result<serde_json::value::Value, bool> {
    let zoomeye_api_url = String::from("https://api.zoomeye.org/host/search");

    let mut params = HashMap::new();
    params.insert("query", query);
    let param_num_str = &page_num.to_string()[..];
    params.insert("page", param_num_str);

    let mut headers = HeaderMap::new();
    headers.insert("API-KEY", token.parse().unwrap());

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
        .get(zoomeye_api_url)
        .query(&params)
        .headers(headers)
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

pub async fn get_socks5_proxy_zoomeye(
    query: &str,
    page_num: i32,
    token: &str,
    search_proxy: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    info!("{}", "Searching from zoomeye...");
    let mut result: Vec<String> = Vec::new();
    for page in 1..page_num + 1 {
        match query_zoomeye(query, page, token, search_proxy).await {
            Ok(data) => {
                for i in data["matches"].as_array().unwrap().iter() {
                    let mut ip_port = String::from("");
                    let ip_str = i["ip"].to_string();
                    let port_str = i["portinfo"]["port"].to_string();
                    ip_port += &ip_str[1..ip_str.len() - 1];
                    ip_port += ":";
                    ip_port += &port_str;
                    result.push(ip_port);
                }
            },
            Err(_) => {
                return Ok(result);
            }
        }
    }
    info!(" - Get {} proxy from {}", result.len(), "zoomeye");
    Ok(result)
}
