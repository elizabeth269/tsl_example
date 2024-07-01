use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::rustls::{self, NoClientAuth, ServerConfig};
use tokio_rustls::TlsAcceptor;

#[tokio::main]
async fn main() {
    // Load server certificates
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    let certs = rustls::internal::pemfile::certs(cert_file).unwrap();
    let mut keys = rustls::internal::pemfile::pkcs8_private_keys(key_file).unwrap();

    // Configure TLS settings
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config));

    // Bind to an address
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let mut stream = acceptor.accept(stream).await.unwrap();
            println!("Accepted connection from {}", addr);

            // Echo received data back to the client
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf).await.unwrap();
            stream.write_all(&buf[0..n]).await.unwrap();
        });
    }
}
