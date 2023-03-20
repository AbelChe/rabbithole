#[forbid(unsafe_code)]

pub mod socks5 {
    use log::{info, warn, error, debug};
    use crate::utils::utils::parse_socks5_url;

    use fast_socks5::{
        server::{Config, SimpleUserPassword, Socks5Server, Socks5Socket},
        client::{self, Socks5Stream},
        // socks4::client::Socks4Stream,
        Result, SocksError,
    };
    use anyhow::{anyhow, Context};
    use tokio::io::{AsyncRead, AsyncWrite};
    use tokio::task;
    use tokio_stream::StreamExt;
    use std::net::ToSocketAddrs;
    use std::io::ErrorKind;

    use rand::seq::SliceRandom;

    pub enum AuthMode {
        NoAuth,
        Password {
            username: String,
            password: String,
        },
    }

    pub async fn spawn_socks_server(request_timeout: u64, auth: AuthMode, listen_host: String, listen_port: u16, proxy_addr_pool: Vec<String>) -> Result<()> {
        let mut config = Config::default();
        config.set_request_timeout(request_timeout);
        config.set_dns_resolve(false);
        config.set_transfer_data(false);

        match auth {
            AuthMode::NoAuth => warn!("No authentication has been set!"),
            AuthMode::Password { username, password } => {
                config.set_authentication(SimpleUserPassword { username, password });
                info!("Simple auth system has been set.");
            }
        }

        let listen_addr = format!("{}:{}", listen_host, listen_port.to_string());

        let mut listener = Socks5Server::bind(&listen_addr).await?;
        listener.set_config(config);
    
        let mut incoming = listener.incoming();
    
        info!("Listen for socks5 connections @ {}, using proxypool[{}]", &listen_addr, proxy_addr_pool.len());

        // Standard TCP loop
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(socket) => {
                    let proxy_addr = proxy_addr_pool.choose(&mut rand::thread_rng()).unwrap().to_string();
                    task::spawn(async move {
                        if let Err(err) = handle_socket(socket, proxy_addr).await {
                            error!("socket handle error = {:#}", err);
                        }
                    });
                }
                Err(err) => {
                    error!("accept error = {:#}", err);
                }
            }
        }
        Ok(())
    }

    async fn handle_socket<T>(socket: Socks5Socket<T>, proxy_addr: String) -> Result<()>
    where
        T: AsyncRead + AsyncWrite + Unpin,
    {
        // upgrade socket to SOCKS5 proxy
        let mut socks5_socket = socket
            .upgrade_to_socks5()
            .await
            .context("upgrade incoming socket to socks5")?;

        // get resolved target addr
        socks5_socket
            .resolve_dns()
            .await
            .context("resolve target dns for incoming socket")?;
        let socket_addr = socks5_socket
            .target_addr()
            .context("find target address for incoming socket")?
            .to_socket_addrs()
            .context("convert target address of incoming socket to socket addresses")?
            .next()
            .context("reach out to target of incoming socket")?;

        // connect to downstream proxy

        let proxy_addr_obj = parse_socks5_url(proxy_addr).unwrap();
        let proxy_schema = proxy_addr_obj.0;
        let proxy_host = proxy_addr_obj.3;
        let proxy_port = proxy_addr_obj.4;



        match proxy_schema.as_str() {
            // socks4 unsupported...
            // "socks4" => {
            //     let mut stream = Socks4Stream::connect(
            //             proxy_addr.clone().as_str()[9..proxy_addr.len()].to_string(),
            //             socket_addr.ip().to_string(),
            //             socket_addr.port(),
            //             false,
            //         )
            //         .await
            //         .context("connect to downstream proxy for incoming socket")?;
            //     match tokio::io::copy_bidirectional(&mut stream, &mut socks5_socket).await {
            //         Ok(res) => {
            //             debug!("socket transfer closed ({}, {})", res.0, res.1);
            //             Ok(())
            //         }
            //         Err(err) => match err.kind() {
            //             ErrorKind::NotConnected => {
            //                 debug!("socket transfer closed by client");
            //                 Ok(())
            //             },
            //             ErrorKind::ConnectionReset => {
            //                 debug!("socket transfer closed by downstream proxy");
            //                 Ok(())
            //             },
            //             _ => Err(SocksError::Other(anyhow!(
            //                 "socket transfer error: {:#}",
            //                 err
            //             )))
            //         },
            //     }
            // },
            "socks5" => {
                let mut stream = Socks5Stream::connect(
                        format!("{}:{}", proxy_host, proxy_port.to_string()),
                        socket_addr.ip().to_string(),
                        socket_addr.port(),
                        client::Config::default(),
                    )
                    .await
                    .context("connect to downstream proxy for incoming socket")?;
                match tokio::io::copy_bidirectional(&mut stream, &mut socks5_socket).await {
                    Ok(res) => {
                        debug!("socket transfer closed ({}, {})", res.0, res.1);
                        Ok(())
                    }
                    Err(err) => match err.kind() {
                        ErrorKind::NotConnected => {
                            debug!("socket transfer closed by client");
                            Ok(())
                        },
                        ErrorKind::ConnectionReset => {
                            debug!("socket transfer closed by downstream proxy");
                            Ok(())
                        },
                        _ => Err(SocksError::Other(anyhow!(
                            "socket transfer error: {:#}",
                            err
                        )))
                    },
                }
            },
            _ => Err(SocksError::Other(anyhow!("unsupported protocol")))
        }
        // copy data between our incoming client and the used downstream proxy
        
    }
}