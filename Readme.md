

<img src="https://img.shields.io/badge/rabbithole-%F0%9F%A6%80%20rust-blueviolet"><img src="https://img.shields.io/github/last-commit/abelche/rabbithole" alt="last-commit" /><img src="https://img.shields.io/github/languages/top/abelche/rabbithole?color=yellow" alt="languages-top" /><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/abelche/rabbithole?color=green">

# 🦀 rabbothole 兔子洞

🦀一个完全使用Rust编写的代理池工具，从网络搜索socks5代理，检测可用性之后开启socks5代理服务。



易于使用

```shell
rabbithole -L socks5://user:pass@0.0.0.0:45678
```



# 🗃 数据来源说明

1. fofa、360quake、zoomeye空间搜索引擎
2. 互联网上公开的代理地址



# 🌟 使用

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




# 🔨 TODO

1. ~~**境**内外IP划分~~
    1) ~~提供 `-z --zone [0(defult), 1, 2]` 参数划分为获取的代理是 所有 、 境内 还是 境外。这是给API搜索的时候提供筛选项~~
    2) ~~在可用性检测的时候同时记录出口IP的位置，根据 `-z --zone [0(defult), 1, 2]` 参数指定的区域进行划分~~

2. 增加API搜索的时效性

3. 增加从文件获取代理地址的功能 `-f --file [filename]`

4. 增加导出文件功能，将搜索获取到的代理地址保存到文件中 `-o --output [filename]`

5. ~~增加 `--delay-test-address [url(defult httpbin.org)]` 参数指定可用性检测地址，`--delay-test-timeout [time(default 5000ms)]`参数指定检测超时事件~~

6. 增加重连功能，当前请求的代理无法使用时候，再使用其他随机代理进行重连，可以使用 `--reconnect [times]` 指定次数

