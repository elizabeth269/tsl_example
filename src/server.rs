use rustls::server::NoClientAuth;
use rustls::HandshakeType::Certificate;
use std::fs::File;
use std::io::{self, BufReader};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;

pub struct TlsServer {
    addr: String,
    cert_path: String,
    key_path: String,
}

impl TlsServer {
    pub fn new(addr: &str, cert_path: &str, key_path: &str) -> Self {
        TlsServer {
            addr: addr.to_string(),
            cert_path: cert_path.to_string(),
            key_path: key_path.to_string(),
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        let certs = load_certs(&self.cert_path)?;
        let key = load_private_key(&self.key_path)?;
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .unwrap();

        let acceptor = TlsAcceptor::from(Arc::new(config));
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", &self.addr);

        loop {
            let (stream, addr) = listener.accept().await?;
            let acceptor = acceptor.clone();

            tokio::spawn(async move {
                let mut stream = acceptor.accept(stream).await.unwrap();
                println!("Accepted connection from {}", addr);

                let mut buf = [0; 1024];
                let n = stream.read(&mut buf).await.unwrap();
                stream.write_all(&buf[0..n]).await.unwrap();
            });
        }
    }
}

fn load_certs(filename: &str) -> io::Result<Vec<Certificate>> {
    let cert_file = &mut BufReader::new(File::open(filename)?);
    rustls::internal::pemfile::certs(cert_file)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid certificate"))
}

fn load_private_key(filename: &str) -> io::Result<PrivateKey> {
    let key_file = &mut BufReader::new(File::open(filename)?);
    let keys = rustls::internal::pemfile::pkcs8_private_keys(key_file)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid private key"))?;
    if keys.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no private keys found",
        ));
    }
    Ok(keys[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task;

    #[tokio::test]
    async fn test_server_creation() {
        let server = TlsServer::new("127.0.0.1:8081", "cert.pem", "key.pem");

        // Run server in a separate task
        let server_task = task::spawn(async move {
            server.run().await.unwrap();
        });

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Shutdown the server
        server_task.abort();
    }
}
