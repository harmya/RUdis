use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use serde::{Deserialize, Serialize};
use httparse;


enum CommandType {
    PING,
    WHAT,
    ERROR
}

impl CommandType {
    fn from_str(command: &str) -> CommandType {
        match command {
            "PING" => CommandType::PING,
            "WHAT" => CommandType::WHAT,
            _ => CommandType::ERROR
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Request {
    message: String
}


fn parse_single_string(data: String) -> String {
    // check first character
    let first_char = data.chars().next().unwrap();
    if first_char == '+' {
        return data[1..].trim_end_matches("\r\n").to_string();
    } else {
        return String::from("Error parsing string");
    }
}

fn execute_command(command: CommandType, data: String) -> String {
    match command {
        CommandType::PING => {
            let pong_message = parse_single_string(data);
            return format!("PONG {}", pong_message);
        },
        CommandType::WHAT => {
            return "This is a implementation of redis using Rust. 
                Checkout my github repo: www.github.com/harmya".to_string();
        },
        CommandType::ERROR => {
            return String::from("Error executing command");
        }
    }
}

fn respond(request: String) -> String {
    let request = request.trim();

    if request.is_empty() {
        return "Error parsing request, no bytes in request body".to_string();
    }

    println!("Message: {}", request);
    let mut request_parts = request.splitn(2, ' ');
    
    let request_command = match request_parts.next() {
        Some(command) => command,
        None => {
            return "Error parsing commands".to_string();
        }
    };

    let request_data = match request_parts.next() {
        Some(data) => data,
        None => {
            return "Error parsing data".to_string();
        }
    };

    println!("Request Command: {}", request_command);
    println!("Request Data: {}", request_data);

    let response = execute_command(CommandType::from_str(request_command), 
        request_data.to_string());

    return response;
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
    let response_body  = respond(body_str.into_owned());

    let status_line = "HTTP/1.1 200 OK\r\n";
    let headers = "Content-Type: text/plain\r\nContent-Length: 256\r\n\r\n";
    let response = format!("{}{}{}", status_line, headers, response_body);

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
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error e:{}", e)
            }
            
        }
    }

}



