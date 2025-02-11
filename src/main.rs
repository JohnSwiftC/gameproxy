use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use rustls::{
    pki_types::ServerName, ClientConfig, ClientConnection, CommonState, RootCertStore,
    ServerConfig, ServerConnection, Stream,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn make_https_request(website: String, request: &[u8]) -> Vec<u8> {
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

    response
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);

    let mut line_buf = String::new();

    if let Err(_) = buf_reader.read_line(&mut line_buf) {
        return;
    }

    let request_parts: Vec<&str> = line_buf.split_whitespace().collect();

    let mut headers = HashMap::new();

    loop {
        let mut line_buf = String::new();

        if let Err(_) = buf_reader.read_line(&mut line_buf) {
            return;
        }

        if line_buf.is_empty() || line_buf == "\n" || line_buf == "\r\n" {
            break;
        }

        let mut comps = line_buf.split(":");
        let key = comps.next().unwrap_or("None");
        let value = comps.next().unwrap_or("None").trim();

        headers.insert(key.to_string(), value.to_string());
    }

    // We need to parse every request going to localhost and proxy it
    // For example, when using cool math games, it needs http://localhost/sites/default/files/2024-09/Simulation.svg
    // so we will request https://www.coolmathgames/sites/default/files/2024-09/Simulation.svg and return it.

    let req = String::new();

    let new_req = format!(
        "GET {} HTTP/1.1\r\nHost: www.coolmathgames.com\r\nConnection: close\r\nAccept-Encoding: identity\r\nUser-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:135.0) Gecko/20100101 Firefox/135.0\r\n\r\n", *request_parts.get(1).expect("Bad Request")
    );

    println!("{}", new_req);

    stream
        .write_all(&make_https_request(
            "www.coolmathgames.com".into(),
            new_req.as_bytes(),
        ))
        .unwrap();
}
