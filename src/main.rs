use tokio::io::AsyncBufRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::Lines;

use std::net::{TcpListener, TcpStream};

use std::sync::Arc;

use rustls::{
    ClientConfig, ClientConnection, CommonState, RootCertStore, ServerConfig, ServerConnection,
    Stream,
};

fn main() {
    println!("Hello, world!");
}

fn make_request(website: String, request: String) {
    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let rc_config = Arc::new(config);
    let example_com = "youtube.com".try_into().unwrap();
    let mut client = ClientConnection::new(rc_config, example_com).unwrap();

    // We have a TLS 'client connection' object that must be combined with an actual TCP stream on port 443 to be turned into a real stream

    let mut sock = TcpStream::connect(&format!("{}:443", website)).unwrap();
    let mut tls_stream = Stream::new(&mut client, &mut sock);
}
