use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use serde::{Deserialize, Serialize};
use httparse;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    message: String
}

fn parse_message_to_response(response: String) -> Request {
    let deserialized: Request = serde_json::from_str(&response).unwrap();
    return deserialized;
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read message in the buffer");
    let http_request = String::from_utf8_lossy(&buffer[..]);
    println!("HTTP Request:\n{}", http_request);

    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();
    if let httparse::Status::Partial = res {
        println!("Incomplete request");
        return;
    }

    let header_len = res.unwrap();
    let body_str = String::from_utf8_lossy(&buffer[header_len..bytes_read]);
    let request_body : Request = parse_message_to_response(body_str.into_owned());

    let status_line = "HTTP/1.1 200 OK\r\n";
    let headers = "Content-Type: text/plain\r\nContent-Length: 12\r\n\r\n";
    let body = "Hello gyatt!";
    let response = format!("{}{}{}", status_line, headers, body);

    stream.write(response.as_bytes()).expect("Failed to send response");
}


fn main() {
    let port = "8080";
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .expect("Failed to bind!");
    println!("Server running on localhost at PORT={}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error e:{}", e)
            }
            
        }
    }

}



