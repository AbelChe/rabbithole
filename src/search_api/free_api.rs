#![allow(warnings)]

use std::{time::Duration, sync::{Arc, Mutex}, thread::JoinHandle};

use futures::{SinkExt, StreamExt};
use log::{info, debug, error};
use reqwest;
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use serde_urlencoded;
use serde_json;
use tokio::time::timeout;
use rayon::prelude::*;

use crate::utils::utils::random_string;


#[derive(Debug, PartialEq, Eq)]
pub enum SearchArea {
    ALL,
    LIMITED,
}

trait Spider {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

// Not blocked by GFW
// https://www.proxy-list.download/SOCKS5
struct Spiderx1;
impl Spider for Spiderx1 {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // implementation for Spiderx1
        let url = String::from("https://www.proxy-list.download/SOCKS5");
        info!(" Search from {}", "proxy-list.download");

        // Spawn an async block to perform async operations
        let res = futures::executor::block_on(async {
            let client = if search_proxy.is_empty() {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap()
            } else {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .proxy(reqwest::Proxy::all(&search_proxy)?)
                    .build()
                    .unwrap()
            };

            client.get(&url).send().await?.text().await
        });

        let mut proxy_list: Vec<String> = Vec::new();
        let res_data = match res {
            Ok(data) => data,
            Err(e) => {
                error!("  {:?}", e);
                return Ok(proxy_list);
            }
        };

        let document = Html::parse_document(&res_data);
        let tr_selector = Selector::parse(r#"#tabli > tr"#).unwrap();
        let td_selector = Selector::parse(r#"td"#).unwrap();

        let trs = document.select(&tr_selector);
        for tr_item in trs {
            let mut tds = tr_item.select(&td_selector);
            let ip = tds.next().unwrap().text().collect::<String>();
            let port = tds.next().unwrap().text().collect::<String>();
            proxy_list.push(format!("socks5://{}:{}", ip.trim(), port.trim()));
        }
        info!("  - Get {} proxy from {}", proxy_list.len(), "proxy-list.download");
        Ok(proxy_list)
    }
}

// Not blocked by GFW
// https://list.proxylistplus.com/Socks-List-1
struct Spiderx2;
impl Spider for Spiderx2 {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // implementation for Spiderx2
        let url = String::from("https://list.proxylistplus.com/Socks-List-1");
        info!(" Search from {}", "proxylistplus.com");

        // Spawn an async block to perform async operations
        let res = futures::executor::block_on(async {
            let client = if search_proxy.is_empty() {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap()
            } else {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .proxy(reqwest::Proxy::all(&search_proxy)?)
                    .build()
                    .unwrap()
            };

            client.get(&url).send().await?.text().await
        });

        let mut proxy_list: Vec<String> = Vec::new();
        let res_data = match res {
            Ok(data) => data,
            Err(e) => {
                error!("  {:?}", e);
                return Ok(proxy_list);
            }
        };

        let document = Html::parse_document(&res_data);
        let tr_selector = Selector::parse(r#"#page > table.bg > tbody > tr.cells"#).unwrap();
        let td_selector = Selector::parse(r#"td"#).unwrap();

        let trs = document.select(&tr_selector);
        for tr_item in trs {
            let mut tds = tr_item.select(&td_selector);
            tds.next();
            let ip = tds.next().unwrap().text().collect::<String>();
            let port = tds.next().unwrap().text().collect::<String>();
            let protocol = tds.next().unwrap().text().collect::<String>();
            if protocol.trim().to_lowercase() == "socks5".to_string() {
                proxy_list.push(format!("socks5://{}:{}", ip.trim(), port.trim()));
            }
        }
        info!("  - Get {} proxy from {}", proxy_list.len(), "proxylistplus.com");
        Ok(proxy_list)
    }
}
// http://proxydb.net/?protocol=socks5&country=

fn find_proxydb_proxy(url: String, search_proxy: String) -> Vec<String> {
    debug!("  searching - {}", url);
    let mut plist: Vec<String> = Vec::new();
    let client = if search_proxy.is_empty() {
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap()
    } else {
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3))
            .proxy(reqwest::Proxy::all(&search_proxy).unwrap())
            .build()
            .unwrap()
    };
    let a_selector = Selector::parse(r#"body > div > div.table-responsive > table > tbody > tr > td > a"#).unwrap();
    let data = match client.get(&url).send() {
        Ok(resp) => match resp.text() {
            Ok(text) => text,
            Err(err) => {
                error!("Failed to get response text: {:?}", err);
                return vec![];
            }
        },
        Err(err) => {
            error!("Failed to get response: {:?}", err);
            return vec![];
        }
    };
    let document_ = Html::parse_document(&data);
    let mut a_nodes = document_.select(&a_selector);
    for a in a_nodes {
        let proxy_addr = a.text().collect::<String>();
        debug!("{}", proxy_addr);
        plist.push(format!("socks5://{}", proxy_addr.trim().to_string()));
    }
    plist
}
struct Spiderx3;
impl Spider for Spiderx3 {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // implementation for Spiderx3
        let url = String::from("http://proxydb.net/?protocol=socks5&country=");
        info!(" Search from {}", "proxydb.net");
        let client = if search_proxy.is_empty() {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(3))
                .pool_max_idle_per_host(10)
                .pool_idle_timeout(Duration::from_secs(10))
                .build()
                .unwrap()
        } else {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(3))
                .proxy(reqwest::Proxy::all(&search_proxy).unwrap())
                .pool_max_idle_per_host(10)
                .pool_idle_timeout(Duration::from_secs(10))
                .build()
                .unwrap()
        };

        let mut proxy_list:Vec<String> = Vec::new();
        // Spawn an async block to perform async operations
        let req = futures::executor::block_on(async {
            client.get(&url).send().await?.text().await
        });

        let res = match req {
            Ok(data) => data,
            Err(err) => {
                debug!("  {:?}", err);
                return Ok(proxy_list)
            }
        };

        let document = Html::parse_document(&res);
        let options_selector = Selector::parse(r#"#country > option"#).unwrap();
        let mut country_list: Vec<String> = Vec::new();
        let mut options = document.select(&options_selector);
        options.next();
        for o in options {
            let c = o.value().attr("value").unwrap().to_owned();
            country_list.push(c);
        }

        let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        country_list.par_iter().for_each(|c| {
            let uurl = format!("{}{}", url, c);
            let proxies = find_proxydb_proxy(uurl, search_proxy.clone());
            let mut result = result.lock().unwrap();
            result.extend(proxies);
            drop(result);
        });

        result.lock().unwrap();
        proxy_list = Arc::try_unwrap(result).unwrap().into_inner().unwrap();

        info!("  - Get {} proxy from {}", proxy_list.len(), "proxydb.net");
        Ok(proxy_list)
    }
}


// Blocked by GFW
// https://hidemy.name/en/proxy-list/?type=5&anon=4
struct Spider1;
impl Spider for Spider1 {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // implementation for Spider1
        let url = String::from("https://hidemy.name/en/proxy-list/?type=5&anon=4");
        info!(" Search from {:?}", url);

        // TODO: Because of this site will verify robot or not...

        Ok(vec![])
    }
}

// Blocked by GFW
// https://spys.one/en/socks-proxy-list/
struct Spider2;
impl Spider for Spider2 {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // implementation for Spider2
        let url = String::from("https://spys.one/en/socks-proxy-list/");
        // let proxy = search_proxy.clone();
        info!(" Search from {}", "spys.one");

        // Spawn an async block to perform async operations
        let res = futures::executor::block_on(async {
            let client = if search_proxy.is_empty() {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap()
            } else {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .proxy(reqwest::Proxy::all(&search_proxy)?)
                    .build()
                    .unwrap()
            };

            let v_xx0 = random_string(32);
            let post_data = [
                    ("xx0", &v_xx0.to_lowercase()[..]),
                    ("xpp", "5"),
                    ("xf1", "0"),
                    ("xf2", "0"),
                    ("xf4", "0"),
                    ("xf5", "2")];
            let post_data_encoded = serde_urlencoded::to_string(&post_data).unwrap();
            client.post(&url).body(post_data_encoded).send().await?.text().await
        });
        
        let mut proxy_list: Vec<String> = Vec::new();
        let res_data = match res {
            Ok(data) => data,
            Err(e) => {
                error!("  {:?}", e);
                return Ok(proxy_list);
            }
        };
        let document = Html::parse_document(&res_data);
        let tr_selector = Selector::parse(r#"body > table:nth-child(3) > tbody > tr:nth-child(4) > td > table > tbody > tr.spy1x , body > table:nth-child(3) > tbody > tr:nth-child(4) > td > table > tbody > tr.spy1xx"#).unwrap(); // select both of tr.spy1x and tr.spy1xx
        let td_selector = Selector::parse(r#"td"#).unwrap();

        let trs = document.select(&tr_selector);
        for tr_item in trs {
            let mut tds = tr_item.select(&td_selector);
            let ip_port = tds.next().unwrap().text().collect::<String>();
            let protocol = tds.next().unwrap().text().collect::<String>();
            if protocol.trim().to_lowercase() == "socks5".to_string() {
                proxy_list.push(format!("socks5://{}", ip_port.trim()));
            }
        }
        info!("  - Get {} proxy from {}", proxy_list.len(), "spys.one");
        Ok(proxy_list)
    }
}

// Blocked by GFW
// https://www.proxydocker.com/en/socks5-list/
struct Spider3;
impl Spider for Spider3 {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // implementation for Spider3
        let url = String::from("https://www.proxydocker.com/en/socks5-list/");
        let api_url = String::from("https://www.proxydocker.com/en/api/proxylist/");
        info!(" Search from {}", "proxydocker.com");

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.9999.0 Safari/537.36".parse().unwrap());

        // Spawn an async block to perform async operations
        let res = futures::executor::block_on(async {
            let client = if search_proxy.is_empty() {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(6))
                    .cookie_store(true)
                    .build()
                    .unwrap()
            } else {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(6))
                    .cookie_store(true)
                    .proxy(reqwest::Proxy::all(&search_proxy[..])?)
                    .build()
                    .unwrap()
            };

            let mut res_list = Vec::new();
            let req1 = client.get(&url).headers(headers.clone()).send().await;
            let res1 = match req1 {
                Ok(res) => res.text().await,
                Err(e) => {
                    error!("  {:?}", e);
                    return Ok::<_, Box<dyn std::error::Error>>(res_list);
                }
            };
            let token = match res1 {
                Ok(text) => {
                    if text == "You are forbidden!".to_string() {
                        error!("  - proxydocker.com returned \"You are forbidden!\"");
                        return Ok::<_, Box<dyn std::error::Error>>(res_list);
                    }
            
                    let document = Html::parse_document(&text);
                    let selector = Selector::parse(r#"meta[name="_token"]"#).unwrap();
                    let meta_token = document.select(&selector).next().unwrap();
                    let token = meta_token.value().attr("content").unwrap().to_owned();
                    if token.is_empty() {
                        return Ok::<_, Box<dyn std::error::Error>>(res_list);
                    } else {
                        token
                    }
                }
                Err(e) => {
                    error!("  {:?}", e);
                    return Ok::<_, Box<dyn std::error::Error>>(res_list);
                },
            };
            debug!("Get token: {:#?}", token);

            headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
            headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());

            for page in 1..=3 {
                let post_data = [
                        ("token", token.as_str()),
                        ("country", "all"),
                        ("city", "all"),
                        ("state", "all"),
                        ("port", "all"),
                        ("type", "socks5"),
                        ("anonymity", "all"),
                        ("need", "all"),
                        ("page", &page.to_string()[..])];
                let post_data_encoded = serde_urlencoded::to_string(&post_data).unwrap();
                let _res = client
                    .post(&api_url)
                    .headers(headers.clone())
                    .body(post_data_encoded)
                    .send()
                    .await?
                    .json::<serde_json::Value>()
                    .await;
                res_list.push(Ok::<_, Box<dyn std::error::Error>>(_res));
            }
            Ok::<_, Box<dyn std::error::Error>>(res_list)
        })?;
        let mut proxy_list: Vec<String> = Vec::new();
        for r in res {
            let i = r.unwrap().unwrap();
            for j in i["proxies"].as_array().unwrap().iter() {
                proxy_list.push(format!("socks5://{}:{}", j["ip"].to_string(), j["port"].to_string()));
            }
        }
        info!("  - Get {} proxy from {}", proxy_list.len(), "proxyscrape.com");
        Ok(proxy_list)
    }
}

// Blocked by GFW
// API!  curl -s 'https://api.proxyscrape.com/proxytable.php?nf=true&country=all' | jq -r '.socks5 | keys'
struct Spider4;
impl Spider for Spider4 {
    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // implementation for Spider4
        let url = String::from("https://api.proxyscrape.com/proxytable.php?nf=true&country=all");
        info!(" Search from {}", "proxyscrape.com");

        // Spawn an async block to perform async operations
        let res = futures::executor::block_on(async {
            let client = if search_proxy.is_empty() {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap()
            } else {
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .proxy(reqwest::Proxy::all(&search_proxy)?)
                    .build()
                    .unwrap()
            };
            client.get(&url).send().await?.json::<serde_json::Value>().await
        });

        let mut proxy_list: Vec<String> = Vec::new();
        match res {
            Ok(json) => {
                if let Some(socks5) = json["socks5"].as_object() {
                    for (k, _) in socks5.iter() {
                        proxy_list.push(format!("socks5://{}", k.clone()));
                    }
                }
            },
            Err(e) => {}
        }
        
        info!("  - Get {} proxy from {}", proxy_list.len(), "proxyscrape.com");
        Ok(proxy_list)
    }
}

struct FreeSocks {
    spiders: Vec<Box<dyn Spider>>,
}

impl FreeSocks {
    fn new(spiders: Vec<Box<dyn Spider>>) -> FreeSocks {
        FreeSocks { spiders }
    }

    fn run(&self, search_proxy: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut r: Vec<String> = Vec::new();
        for spider in &self.spiders {
            let s_proxy = search_proxy.clone();
            r.extend(spider.run(s_proxy)?);
        }
        Ok(r)
    }
}

pub fn get_socks5_proxy_freesite(
    search_area: SearchArea,
    search_proxy: String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    info!("Search from free sites...");
    let r = if search_area == SearchArea::ALL {
        let spiders = FreeSocks::new(vec![
            Box::new(Spiderx1),
            Box::new(Spiderx2),
            Box::new(Spiderx3),
            // Box::new(Spider1), // because of robot verify
            Box::new(Spider2),
            Box::new(Spider3),
            Box::new(Spider4),
        ]);
        spiders.run(search_proxy)?
    } else if search_area == SearchArea::LIMITED {
        let spiders = FreeSocks::new(vec![
            Box::new(Spiderx1),
            Box::new(Spiderx2),
            Box::new(Spiderx3),
        ]);
        spiders.run(search_proxy)?
    } else {
        Vec::new()
    };
    info!(" - Get {} proxy from free sites", r.len());
    Ok(r)
}
