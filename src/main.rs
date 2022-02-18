use std::{error::Error, net::SocketAddr, sync::Arc};

use rustls;
use tokio;
use quinn::{ClientConfig, Endpoint, Incoming, ServerConfig};

use futures_util::StreamExt;

pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error>> {
    let client_cfg = configure_client(server_certs)?;
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

pub fn make_server_endpoint(bind_addr: SocketAddr) -> Result<(Incoming, Vec<u8>), Box<dyn Error>> {
    let (server_config, server_cert) = configure_server()?;
    let (_endpoint, incoming) = Endpoint::server(server_config, bind_addr)?;
    Ok((incoming, server_cert))
}

fn configure_client(server_certs: &[&[u8]]) -> Result<ClientConfig, Box<dyn Error>> {
    let mut certs = rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(&rustls::Certificate(cert.to_vec()))?;
    }
    Ok(ClientConfig::with_root_certificates(certs))
}

fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
    Arc::get_mut(&mut server_config.transport)
        .unwrap()
        .max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let (mut incoming, server_cert) = make_server_endpoint(server_addr)?;

    tokio::spawn(async move {
        // accept a single connection
        let incoming_conn = incoming.next().await.unwrap();
        let new_conn = incoming_conn.await.unwrap();
        println!(
            "[server] connection accepted: addr={}",
            new_conn.connection.remote_address()
        );

    let (mut send, _recv) = new_conn.connection.open_bi().await.unwrap();
        println!("[server] here");
        send.write_all(b"test").await.unwrap();
        send.finish().await.unwrap();
        println!("[server] peace.");
    });

    let endpoint = make_client_endpoint("0.0.0.0:0".parse().unwrap(), &[&server_cert])?;
    // connect to server
    let quinn::NewConnection {
        connection,
        mut bi_streams,
        ..
    } = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    println!("[client] connected: addr={}", connection.remote_address());

    let (_send, recv) = bi_streams.next().await.unwrap().unwrap();
    let received = recv.read_to_end(10).await?;
    println!("[client] {:?} {}", received, std::str::from_utf8(&received).unwrap());

    // Give the server has a chance to clean up
    endpoint.wait_idle().await;

    Ok(())
}
