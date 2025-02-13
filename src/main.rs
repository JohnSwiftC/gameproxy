use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use rustls::{
    pki_types::ServerName, ClientConfig, ClientConnection, CommonState, RootCertStore,
    ServerConfig, ServerConnection, Stream,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let currentSite = Arc::new(Mutex::new(String::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let currentSite = Arc::clone(&currentSite);

        thread::spawn(move || {
            handle_connection(stream, currentSite);
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

fn handle_connection(mut stream: TcpStream, mut currentSite: Arc<Mutex<String>>) {
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

    // Braces added so we drop the lock after cloning
    let tag = { currentSite.lock().unwrap().clone() };

    // Check for special routes

    match request_parts.get(1) {
        Some(route) => {
            if *route == "/resetroute" {
                let mut currentSite = currentSite.lock().unwrap();
                *currentSite = "".to_string();

                let status_line = "HTTP/1.1 200 OK";
                let contents = fs::read_to_string("connect.html").unwrap();
                let length = contents.len();

                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                stream.write_all(response.as_bytes()).unwrap();

                return;
            } else {
                let mut comps = route.split("/");
                let _ = comps.next();
                match comps.next() {
                    Some(root) => {
                        if root == "changesite" {
                            match comps.next() {
                                Some(site) => {
                                    let mut currentSite = currentSite.lock().unwrap();
                                    *currentSite = site.to_string();

                                    let status_line = "HTTP/1.1 200 OK";
                                    let contents = fs::read_to_string("connect.html").unwrap();
                                    let length = contents.len();

                                    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                                    stream.write_all(response.as_bytes()).unwrap();

                                    return;
                                }
                                None => (),
                            }
                        }
                    }
                    None => (),
                }
            }
        }
        None => (),
    }

    // Current site is currently established
    if tag != "" {
        let mut req = String::new();
        req.push_str(&line_buf); // Add request line

        headers.insert("Host".to_string(), tag.clone());
        headers.insert("Connection".to_string(), "close".to_string());
        //headers.insert("Accept-Encoding".to_string(), "identity".to_string());
        headers.remove("Site-Tag");

        // We can take ownership of the string here because we want to
        for (header, value) in headers {
            req.push_str(&format!("{}: {}", header, value));
            req.push_str("\r\n");
        }

        req.push_str("\r\n");

        stream
            .write_all(&make_https_request(tag.clone(), req.as_bytes()))
            .unwrap();
    } else {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("connect.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    //println!("{}", req);
}
