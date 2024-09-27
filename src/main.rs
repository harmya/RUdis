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
            return format!("PONG {}\n", pong_message);
        },
        CommandType::WHAT => {
            return "This is a implementation of redis using Rust. Checkout my github repo: www.github.com/harmya\n".to_string();
        },
        CommandType::ERROR => {
            return String::from("Error executing command\n");
        }
    }
}

fn respond(request: String) -> String {
    let request = request.trim();

    if request.is_empty() {
        return "Error parsing request, no bytes in request body\n".to_string();
    }

    // println!("Message: {}", request);
    let mut request_parts = request.splitn(2, ' ');
    
    let request_command = match request_parts.next() {
        Some(command) => command,
        None => {
            return "Error parsing commands\n".to_string();
        }
    };
    
    let request_data = request_parts.next().unwrap_or("");

    // println!("Request Command: {}", request_command);
    // println!("Request Data: {}", request_data);

    return execute_command(CommandType::from_str(request_command), request_data.to_string());
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read message in the buffer");
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request:\n{}", request);

    let response  = respond(request.into_owned());

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