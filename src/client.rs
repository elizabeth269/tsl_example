use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::error;
use tokio_rustls::rustls::{pki_types::ServerName, ClientConfig, RootCertStore};
use tokio_rustls::TlsConnector;
use webpki_roots::TLS_SERVER_ROOTS;

pub struct TlsClient {
    addr: String,
    domain: String,
}

impl TlsClient {
    pub fn new(addr: &str, domain: &str) -> Self {
        TlsClient {
            addr: addr.to_string(),
            domain: domain.to_string(),
        }
    }

    pub async fn run(&self) -> std::io::Result<(rustls::ClientConnection, rustls::Error)> {
        // let mut root_store = RootCertStore::empty();
        // root_store.add_server_trust_anchors(&TLS_SERVER_ROOTS);
        let root_store =
            rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        // let config = ClientConfig::builder()
        //     .with_safe_defaults()
        //     .with_root_certificates(root_store)
        //     .with_no_client_auth();
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        // let connector = TlsConnector::from(Arc::new(config));
        // let stream = TcpStream::connect(&self.addr).await?;
        // let domain = ServerName::try_from(self.domain.as_str()).unwrap();
        // let mut stream = connector.connect(domain, stream).await.unwrap();

        let rc_config = Arc::new(config);
        let domain = ServerName::try_from(self.domain.as_str()).unwrap();
        let mut stream = rustls::ClientConnection::new(rc_config, domain);

        stream.write_al(b"Hello, server!").await.unwrap();
        let mut buf = [0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buf[0..n]));

        Ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task;

    #[tokio::test]
    async fn test_client_creation() {
        let client = TlsClient::new("127.0.0.1:8081", "localhost");

        // Attempt to run the client
        let client_task = task::spawn(async move {
            client.run().await.unwrap();
        });

        // Give the client a moment to attempt connection
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Shutdown the client
        client_task.abort();
    }
}
