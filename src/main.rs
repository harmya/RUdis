use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


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


fn require_data(data: String) -> String {
    if data.is_empty() {
        return String::from("");
    } else {
        // if data has single quotes, remove them
        if data.starts_with('\'') && data.ends_with('\'') {
            return data[1..data.len()-1].to_string();
        } else {
            return data;
        }
    }
}

fn execute_command(command: CommandType, data: String) -> String {
    match command {
        CommandType::PING => {
            let pong_message = require_data(data);
            return format!("PONG {}\n", pong_message);
        },
        CommandType::WHAT => {
            return "This is a implementation of redis using Rust. Checkout my github: www.github.com/harmya\n".to_string();
        },
        CommandType::ERROR => {
            return String::from("Error executing command, not found\n");
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
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read message in the buffer");
        if bytes_read == 0 {
            break;
        }
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Request:\n{}", request);
        let response  = respond(request.into_owned());
        stream.write(response.as_bytes()).expect("Failed to send response");
    }
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