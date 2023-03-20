use clap::Parser;

// const ABOUT: &str = "Searching public socks5 agents on the Internet, and start proxy pool service.";
pub const LONG_ABOUT: &str = r#"
Searching public socks5 agents on the Internet, and start proxy pool service.
  Examples:
  # Searching socks5 proxy from fofa, zoomeye, quake and all others, and open proxy pool service on 0.0.0.0:7777 with authentication.
  rabbithole -L socks5://user:password@0.0.0.0:7777 --fofa-email <FOFA_EMAIL> --fofa-token <FOFA_TOKEN> --zoomeye_token <ZOOMEYE_TOKEN> --quake-token <QUAKE_TOKEN>

  # Just searching socks5 proxy from free, and open socks5 proxy pool server on 0.0.0.0:7777 WITHOUT authentication.
  rabbithole -L socks5://0.0.0.0:7777
  # Open socks5 proxy pool server on 0.0.0.0:7777 WITH authentication.
  rabbithole -L socks5://user:password@0.0.0.0:7777

  # Of course, '--search-proxy' param could be set for searching from APIs.
  rabbithole --search-proxy socks5://127.0.0.1:7890
"#;

#[derive(Parser, Debug)]
#[command(author = "AbelChe", version = "1.0.0", about = LONG_ABOUT)]
pub struct Args {
    /// Fofa email used by fofa api searching,
    #[arg(long)]
    pub fofa_email: Option<String>,
    /// Fofa API-token used by fofa api searching
    #[arg(long)]
    pub fofa_token: Option<String>,
    /// How many pieces of data to search on fofa
    #[arg(long, default_value_t = 300)]
    pub fofa_size: i32,

    /// Zoomeye token used by zoomeye api searching
    #[arg(long)]
    pub zoomeye_token: Option<String>,
    /// How many pages to search on zoomeye, 20 pieces of data per page
    #[arg(long, default_value_t = 5)]
    pub zoomeye_page_size: i32,

    /// Quake API-token used by quake api searching
    #[arg(long)]
    pub quake_token: Option<String>,
    /// How many pieces of data to search on quake
    #[arg(long, default_value_t = 200)]
    pub quake_size: i32,

    /// Proxy setting, need to be set as socks5://[user:[password@]]proxyhost:port
    #[arg(short = 'L', long, default_value_t = String::from("socks5://0.0.0.0:7777"))]
    pub listen: String,

    /// Log level (debug, info, warn, error, trace)
    #[arg(short, long, default_value_t = String::from("info"))]
    pub level: String,

    /// Proxy for Searching from APIs
    #[arg(long)]
    pub search_proxy: Option<String>,

    /// Zone of proxy, such as [0]inland-CN, [1]outside-CN(just HK,TW and MO), [2]exclude-CN(exclude CN,HK,TW and MO), [3]all-CN, [4]all
    #[arg(short, long, default_value_t = 4i8)]
    pub zone: i8,

    /// Delay testing address
    #[arg(long, default_value_t = String::from("http://httpbin.org/ip"))]
    pub delay_test_address: String,

    /// Delay testing timeout, in milliseconds, such as `--delay-test-timeout 2000`
    #[arg(long, default_value_t = 5000)]
    pub delay_test_timeout: u64,
}
