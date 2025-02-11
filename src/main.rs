use tokio::io::AsyncBufRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::Lines;

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

async fn make_request(website: String, request: String) {
    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let rc_config = Arc::new(config);
    let example_com = "youtube.com".try_into().unwrap();
    let mut client = rustls::ClientConnection::new(rc_config, example_com);
}
