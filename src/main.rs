mod search_api;
mod utils;
mod socksserver;

use clap::Parser;
use utils::cli::Args;

use std::collections::HashSet;
use std::time::Duration;
use search_api::free_api::{get_socks5_proxy_freesite, SearchArea};
use search_api::{
    fofa::get_socks5_proxy_fofa, quake::get_socks5_proxy_quake, zoomeye::get_socks5_proxy_zoomeye,
};
use utils::check::{test_connect, test_connect_google};
use utils::utils::{Socks5Proxy, parse_socks5_url};
use socksserver::socks5::{AuthMode, spawn_socks_server};

use tokio;

use log::{self, LevelFilter};
use log::{error, info, warn};

use env_logger::fmt::Color;
use env_logger::{Builder, Env};

use std::fmt;
use std::io::Write;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INF"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERR"),
        }
    }
}

const VERSION: &str = "v1.1.0";

fn banner() {
    println!(
        r#"
                __    __    _ __  __          __
    /\_/\____ _/ /_  / /_  (_) /_/ /_  ____  / /__
   / ___/ __ `/ __ \/ __ \/ / __/ __ \/ __ \/ / _ \
  / /  / /_/ / /_/ / /_/ / / /_/ / / / /_/ / /  __/
 /_/   \__,_/_.___/_.___/_/\__/_/ /_/\____/_/\___/ {version}

       https://github.com/abelche/rabbithole
"#,
        version = VERSION
    );
}

#[tokio::main]
async fn main() {
    banner();

    let args = Args::parse();
    let log_level = match &args.level.to_string()[..] {
        "info" =>  LevelFilter::Info,
        "debug" => {
            println!("set log level: {}", "debug");
            LevelFilter::Debug
        }
        "warn" => {
            println!("set log level: {}", "warn");
            LevelFilter::Warn
        }
        "error" => {
            println!("set log level: {}", "error");
            LevelFilter::Error
        }
        "trace" => {
            println!("set log level: {}", "trace");
            LevelFilter::Trace
        }
        _ => {
            println!("Log level need to set during (debug, info, warn, error, trace) [default: info]");
            std::process::exit(1)
        },
    };

    Builder::from_env(Env::default().default_filter_or("off"))
        .filter_module("rabbithole", log_level)
        .filter_module("html5ever", LevelFilter::Off)
        .filter_module("scraper", LevelFilter::Off)
        .filter_module("selectors", LevelFilter::Off)
        .format(|buf, record| {
            let mut style = buf.style();
            match record.level() {
                log::Level::Trace => style.set_color(Color::White),
                log::Level::Debug => style.set_color(Color::Magenta),
                log::Level::Info => style.set_color(Color::Blue),
                log::Level::Warn => style.set_color(Color::Yellow),
                log::Level::Error => style.set_color(Color::Red),
            };
            let level = record.level();
            let level = match level {
                log::Level::Trace => LogLevel::Trace,
                log::Level::Debug => LogLevel::Debug,
                log::Level::Info => LogLevel::Info,
                log::Level::Warn => LogLevel::Warning,
                log::Level::Error => LogLevel::Error,
            };
            match level {
                LogLevel::Trace => writeln!(buf, "[{} {}] {}", style.value(level), record.target(), record.args()),
                LogLevel::Debug => writeln!(buf, "[{} {}] {}", style.value(level), record.target(), record.args()),
                LogLevel::Info => writeln!(buf, "[{}] {}", style.value(level), record.args()),
                LogLevel::Warning => writeln!(buf, "[{}] {}", style.value(level), record.args()),
                LogLevel::Error => writeln!(buf, "[{}] {}", style.value(level), record.args()),
            }
        })
        .init();

    // parse proxy setting
    let socks5_url = args.listen.clone();
    if !socks5_url.starts_with("socks5://") {
        error!("Proxy setting error, -L shoule be set as `-L socks5://user:password@0.0.0.0:7777` use -h for more help");
        std::process::exit(1);
    }
    let socks5proxy = parse_socks5_url(socks5_url).unwrap();
    let socks5 = Socks5Proxy::new(socks5proxy.0, socks5proxy.1, socks5proxy.2, socks5proxy.3, socks5proxy.4);
    let auth = match socks5.need_auth() {
        true => {
            let username = socks5.username;
            let password = socks5.password;
            AuthMode::Password { username, password }
        },
        false => {
            AuthMode::NoAuth
        }
    };

    let mut r: Vec<String> = Vec::new();

    let mut search_area: SearchArea = SearchArea::LIMITED; // could be set ALL or LIMITED
    let search_proxy = if args.search_proxy.is_some() {
        args.search_proxy.unwrap()
    } else {
        String::from("")
    };
    let s_proxy = search_proxy.clone();
    if !test_connect_google(search_proxy).await.is_err() {
        info!("The network is not blocked!");
        search_area = SearchArea::ALL;
    } else {
        warn!("The network is blocked, cannot connect to www.google.com.");
    }

    let r0 = get_socks5_proxy_freesite(search_area, s_proxy.clone()).unwrap();
    r.extend(r0);

    if (!args.fofa_email.is_some() && args.fofa_token.is_some())
        || (args.fofa_email.is_some() && !args.fofa_token.is_some())
    {
        let fofahelpinfo = r#"Please set fofa_email and fofa_token at the same time!
    rabbithole --fofa-email <FOFA_EMAIL> --fofa-token <FOFA_TOKEN>"#;
        error!("{}", fofahelpinfo);
        std::process::exit(1);
    }

    if let Some(fofa_email) = args.fofa_email.as_deref() {
        if let Some(fofa_token) = args.fofa_token.as_deref() {
            let fofa_query = "protocol=\"socks5\" && \"Version:5 Method:No Authentication(0x00)\" && country=\"CN\"";
            let fofa_query_size = args.fofa_size;
            let r1 = get_socks5_proxy_fofa(fofa_query, fofa_query_size, fofa_email, fofa_token, s_proxy.clone().as_str())
                .await
                .unwrap();
            r.extend(r1);
        }
    }

    if let Some(zoomeye_token) = args.zoomeye_token.as_deref() {
        let zoomeye_query = "service:\"socks5\" +banner:\"Version:5 Method:No Authentication(0x00)\" +country:\"CN\"";
        let zoomeye_page_num = args.zoomeye_page_size;
        let r2 = get_socks5_proxy_zoomeye(zoomeye_query, zoomeye_page_num, zoomeye_token, s_proxy.clone().as_str())
            .await
            .unwrap();
        r.extend(r2);
    }

    if let Some(quake_token) = args.quake_token.as_deref() {
        let quake_query = "service:\"socks5\" AND response:\"Version: 5 Accepted Auth Method: 0x0 (No authentication)\" AND country: \"China\"";
        let quake_query_size = args.quake_size;
        let r3 = get_socks5_proxy_quake(quake_query, quake_query_size, quake_token, s_proxy.clone().as_str())
            .await
            .unwrap();
        r.extend(r3);
    }

    let uniqued_proxy_list = r
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    match uniqued_proxy_list.len() {
        0 => {
            warn!("Get 0 proxy address from searching, exiting...");
            std::process::exit(1);
        },
        _ => info!(
            "Get {:?} proxy address from searching, now checking availability.",
            uniqued_proxy_list.len())
    }

    let zone = match args.zone {
        0i8 => 0i8,
        1i8 => 1i8,
        2i8 => 2i8,
        3i8 => 3i8,
        4i8 => 4i8,
        _ => {
            warn!("`-z -zone` value set error, should be set in 0 1 2 or 3, \
            [0]inland-CN, [1]outside-CN(just HK,TW and MO), [2]exclude-CN(exclude CN,HK,TW and MO), [3]all-CN, [4]all. \
            But found the value {} has been set. Now use default value: [4]all.", args.zone.clone().to_string());
            4i8
        }
    };

    let delay_test_address = args.delay_test_address;
    let time_out = Duration::from_millis(args.delay_test_timeout);

    let available_proxy = test_connect(
        uniqued_proxy_list,
        delay_test_address,
        time_out,
        zone
    ).await.unwrap();

    info!(
        "Finally got {:?} available proxy address",
        available_proxy.len()
    );

    // start proxy server
    match available_proxy.len() {
        0 => warn!("Because of not found available proxy, exit..."),
        _ => {
            let _ = spawn_socks_server(8u64, auth, socks5.host, socks5.port as u16, available_proxy).await;
        }
    }
}
