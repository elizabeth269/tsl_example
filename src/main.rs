use tls_library::client::TlsClient;
use tls_library::server::TlsServer;
use tokio::task;

#[tokio::main]
async fn main() {
    let server = TlsServer::new("127.0.0.1:8080", "cert.pem", "key.pem");
    let client = TlsClient::new("127.0.0.1:8080", "localhost");

    // Run server and client concurrently
    task::spawn(async {
        server.run().await.unwrap();
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    client.run().await.unwrap();
}
