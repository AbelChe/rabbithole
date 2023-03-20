use futures::{stream, StreamExt};
use log::{debug, info};
use serde_json;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Set Max count of request
const PARALLEL_REQUESTS: usize = 32;

// - get proxy physical location
// - test proxy delay
async fn check_availability(
    proxy_address: String,
    delay_test_address: String,
    time_out: Duration,
) -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
    let test_url = String::from("http://ipinfo.io");
    debug!("Now checking {}", proxy_address);

    let proxy = reqwest::Proxy::all(proxy_address.clone())?;

    let client1 = reqwest::Client::builder()
        .proxy(proxy.clone())
        .timeout(time_out)
        .build()
        .unwrap();

    let client2 = reqwest::Client::builder()
        .proxy(proxy.clone())
        .timeout(time_out)
        .build()
        .unwrap();

    let res = client1
        .get(test_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await;

    // delay test
    let _ = client2.get(&delay_test_address).send().await?;

    match res {
        Ok(res) => {
            let ip = match res["ip"].as_str() {
                Some(ip) => ip.trim_matches('"'),
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Can not find ip address from response",
                    )))
                }
            };
            let city = match res["city"].as_str() {
                Some(city) => city.trim_matches('"'),
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Can not find city from response",
                    )))
                }
            };
            let country = match res["country"].as_str() {
                Some(country) => country.trim_matches('"'),
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Can not find country from response",
                    )))
                }
            };
            Ok((proxy_address.clone(), ip.to_string(), city.to_string(), country.to_string()))
        },
        Err(err) => {
            debug!("{:?}", err);
            return Err(Box::new(err))
        }
    }
}

// Test the availability of the socks5 proxy
pub async fn test_connect(
    proxy_list: Vec<String>,
    delay_test_address: String,
    time_out: Duration,
    zone: i8,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    info!("{}", "Test connect...");
    let pl = proxy_list.clone();
    let z = zone.clone();
    let d = delay_test_address;
    let t = time_out;
    let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let responses = stream::iter(pl)
        .map(|i| {
            let result = Arc::clone(&result);
            // let d = Arc::clone(&d);
            // let t = Arc::clone(&t);
            let d = d.clone();
            let t = t.clone();
            async move {
                let resp = check_availability(i, d, t).await;
                let mut guard = result.lock().unwrap();
                match resp {
                    Ok(i) => {
                        // `i.0` is `proxy_address` as socks5://206.189.157.253:59166
                        // `i.1` is `ip` as 206.189.157.253, maybe not as the same as i.0
                        // `i.2` is `city` as Singapore
                        // `i.3` is `country` as SG
                        match z {
                            // Inland China NOT include HK TW MO
                            0i8 => {
                                if i.3.to_uppercase().as_str() != "CN" {return }
                            },
                            // Only include HK MO TW
                            1i8 => {
                                if !vec!["HK", "MO", "TW"].contains(&i.3.to_uppercase().as_str()) {return }
                            },
                            // All zone of China both of 0 and 1
                            2i8 => {
                                if !vec!["CN", "HK", "MO", "TW"].contains(&i.3.to_uppercase().as_str()) {return }
                            },
                            // Include CN HK MO TW
                            3i8 => {
                                if vec!["CN", "HK", "MO", "TW"].contains(&i.3.to_uppercase().as_str()) {return }
                            },
                            // All of the world [defult]
                            4i8 => { },
                            _ => { },
                        }
                        info!(
                            "Find available proxy: {}, out to: {} @ {}[{}]",
                            i.0.clone(),
                            i.1,
                            i.2,
                            i.3,
                        );
                        guard.push(i.0);
                    }
                    Err(_e) => {
                        debug!("{:?}", _e);
                    }
                }
            }
        })
        .buffer_unordered(PARALLEL_REQUESTS); // set Max request count to PARALLEL_REQUESTS

    responses.for_each(|_| async {}).await;
    let guard = result.lock().unwrap();
    // let guard = result.into_inner().unwrap();
    Ok(guard.clone())
}

pub async fn test_connect_google(proxy_address: String) -> Result<(), reqwest::Error> {
    info!("Testing the network for blockages...");
    let test_url = String::from("https://www.google.com");

    let client = if proxy_address.is_empty() {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?
    } else {
        reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(&proxy_address[..])?)
            .timeout(Duration::from_secs(5))
            .build()?
    };

    client.get(test_url).send().await?;

    Ok(())
}
