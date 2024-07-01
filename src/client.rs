use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::rustls::{self, ClientConfig};
use tokio_rustls::TlsConnector;
use webpki_roots::TLS_SERVER_ROOTS;

#[tokio::main]
async fn main() {
    // Configure TLS settings
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_no_client_auth();
    config
        .root_store
        .add_server_trust_anchors(&TLS_SERVER_ROOTS);

    let connector = TlsConnector::from(Arc::new(config));

    // Connect to the server
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let domain = rustls::ServerName::try_from("localhost").unwrap();
    let mut stream = connector.connect(domain, stream).await.unwrap();

    // Send a message to the server
    stream.write_all(b"Hello, server!").await.unwrap();

    // Read the response from the server
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf[0..n]));
}
