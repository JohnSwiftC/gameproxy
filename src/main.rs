use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use rustls::{
    pki_types::ServerName, ClientConfig, ClientConnection, CommonState, RootCertStore,
    ServerConfig, ServerConnection, Stream,
};

fn main() {
    println!(
        "{}",
        make_https_request(
            "www.rust-lang.com".into(),
            concat!(
                "GET / HTTP/1.1\r\n",
                "Host: www.rust-lang.org\r\n",
                "Connection: close\r\n",
                "Accept-Encoding: identity\r\n",
                "\r\n"
            )
            .as_bytes(),
        )
    );
}

fn make_https_request(website: String, request: &[u8]) -> String {
    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let rc_config = Arc::new(config);
    let example_com = ServerName::try_from(website.clone()).expect("Bad DNS");
    let mut client = ClientConnection::new(rc_config, example_com).unwrap();

    // We have a TLS 'client connection' object that must be combined with an actual TCP stream on port 443 to be turned into a real stream

    let mut sock = TcpStream::connect(&format!("{}:443", website)).unwrap();
    let mut tls_stream = Stream::new(&mut client, &mut sock);

    tls_stream.write_all(request).unwrap();

    let mut response: Vec<_> = Vec::new();

    tls_stream.read_to_end(&mut response).unwrap();

    String::from_utf8(response).expect("Bad Bytes")
}
