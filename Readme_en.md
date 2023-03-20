

<img src="https://img.shields.io/badge/rabbithole-%F0%9F%A6%80%20rust-blueviolet" />  <img src="https://img.shields.io/github/last-commit/abelche/rabbithole" alt="last-commit" />  <img src="https://img.shields.io/github/languages/top/abelche/rabbithole?color=yellow" alt="languages-top" />  <img alt="GitHub repo size" src="https://img.shields.io/github/languages/code-size/abelche/rabbithole?color=green">

# 🦀 rabbithole

[中文](./Readme.md)  |  [English](./Readme_en.md)



🦀A proxy pool tool completely written in Rust, which searches for SOCKS5 proxies from the network, and after checking their availability, starts a SOCKS5 proxy service.



Easy to use

```shell
rabbithole -L socks5://user:pass@0.0.0.0:45678
```



# 🗃 Data source description

1. Spatial search engine, as fofa, 360quake, zoomeye
2. Publicly available proxy addresses on the internet



# 🌟 Usage

```
$ rabbithole -h

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


Usage: rabbithole [OPTIONS]

Options:
      --fofa-email <FOFA_EMAIL>
          Fofa email used by fofa api searching,
      --fofa-token <FOFA_TOKEN>
          Fofa API-token used by fofa api searching
      --fofa-size <FOFA_SIZE>
          How many pieces of data to search on fofa [default: 300]
      --zoomeye-token <ZOOMEYE_TOKEN>
          Zoomeye token used by zoomeye api searching
      --zoomeye-page-size <ZOOMEYE_PAGE_SIZE>
          How many pages to search on zoomeye, 20 pieces of data per page [default: 5]
      --quake-token <QUAKE_TOKEN>
          Quake API-token used by quake api searching
      --quake-size <QUAKE_SIZE>
          How many pieces of data to search on quake [default: 200]
  -L, --listen <LISTEN>
          Proxy setting, need to be set as socks5://[user:[password@]]proxyhost:port [default: socks5://0.0.0.0:7777]
  -l, --level <LEVEL>
          Log level (debug, info, warn, error, trace) [default: info]
      --search-proxy <SEARCH_PROXY>
          Proxy for Searching from APIs
      --check-url <CHECK_URL>
          Checking URL for availability testing
  -z, --zone <ZONE>
          Zone of proxy, such as [0]inland-CN, [1]outside-CN(just HK,TW and MO), [2]exclude-CN(exclude CN,HK,TW and MO), [3]all-CN, [4]all [default: 4]
      --delay-test-address <DELAY_TEST_ADDRESS>
          Delay testing address [default: http://httpbin.org/ip]
      --delay-test-timeout <DELAY_TEST_TIMEOUT>
          Delay testing timeout, in milliseconds, such as `--delay-test-timeout 2000` [default: 5000]
  -h, --help
          Print help
  -V, --version
          Print version
```



![image-20230320143950045](resource/image-20230320143950045.png)



![image-20230320144206346](resource/image-20230320144206346.png)


