use rand::Rng;
use url::Url;
use percent_encoding::percent_decode;

pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}


pub fn parse_socks5_url(url: String) -> Result<(String, String, String, String, u16), Box<dyn std::error::Error>> {
    let _url = Url::parse(&url).expect("Failed to parse URL");

    // 提取 scheme、username、password、hostname、port 等信息
    let scheme = _url.scheme().to_string();
    let username = percent_decode(_url.username().as_bytes()).decode_utf8().unwrap().to_string();
    let password = percent_decode(_url.password().unwrap_or_default().as_bytes()).decode_utf8().unwrap().to_string();
    let host = _url.host_str().expect("Hostname not found").to_string();
    let port = _url.port().expect("Port not found");

    Ok((scheme, username, password, host, port))
}

pub struct Socks5Proxy {
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl Socks5Proxy {
    pub fn new(scheme: String, username: String, password: String, host: String, port: u16) -> Self { Self { scheme, username, password, host, port } }

    pub fn need_auth(&self) -> bool {
        match !self.username.is_empty() && !self.password.is_empty() {
            true => true,
            false => false,
        }
    }
}