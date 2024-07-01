# TLS Library

A simple TLS library in Rust for secure communication over the internet. This library supports TLS 1.2 and TLS 1.3, secure key exchange, and certificate handling. It provides both server and client functionalities using the `rustls` and `tokio-rustls` crates.

## Features

- Support for TLS 1.2 and TLS 1.3
- Secure key exchange and certificate handling
- Integration with web servers and clients
- Example implementations of a TLS server and client

## Prerequisites

- Rust and Cargo installed
- OpenSSL installed (for generating certificates)

## Usage

### Generating Certificates

Generate a self-signed certificate and a private key using OpenSSL:

```sh
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout key.pem -out cert.pem -subj "/CN=localhost"
```

#### Running the Server and Client

Clone the repository: git clone https://github.com/elizabethadebayo269/tls_example.git
cd tls_example

Build and run the project: cargo run

#### Project Structure

src/lib.rs: Exposes the server and client modules.
src/server.rs: Contains the implementation of the TLS server.
src/client.rs: Contains the implementation of the TLS client.

## Running Tests

To run the tests, use the following command: cargo test
