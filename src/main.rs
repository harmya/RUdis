use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[derive(Debug)]
enum CommandType {
    PING,
    WHAT,
    ECHO,
    SET,
    GET,
    ERROR
}

impl CommandType {
    fn from_str(command: &str) -> CommandType {
        match command {
            "PING" => CommandType::PING,
            "WHAT" => CommandType::WHAT,
            "ECHO" => CommandType::ECHO,
            "SET" => CommandType::SET,
            "GET" => CommandType::GET,
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

fn execute_command(command: CommandType, data: String, storage: &mut std::collections::HashMap<String, String>) -> String {
    println!("Executing command: {:?} with data: {}", command, data);
    match command {
        CommandType::ECHO => {
            let pong_message = require_data(data);
            return format!("{}\n", pong_message);
        },
        CommandType::PING => {
            return "PONG\n".to_string();
        },
        CommandType::WHAT => {
            return "This is a implementation of redis using Rust. Checkout my github: www.github.com/harmya\n".to_string();
        },
        CommandType::SET => {
            let mut data_parts = data.splitn(2, ' ');
            let key = data_parts.next().unwrap_or("");
            let value = data_parts.next().unwrap_or("");
            let key = require_data(key.to_string());
            let value = require_data(value.to_string());
            let response = format!("Success");
            storage.insert(key, value).expect("Failed to insert key value pair");
            return response;
        },
        CommandType::GET => {
            let key = require_data(data);
            let value = storage.get(&key);
            match value {
                Some(value) => {
                    return format!("Value: {}\n", value);
                },
                None => {
                    return format!("Value: Not found\n");
                }
            }
        },
        CommandType::ERROR => {
            return String::from("Error executing command, not found\n");
        }
    }
}

fn respond(request: String, storage: &mut std::collections::HashMap<String, String>) -> String {
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

    return execute_command(CommandType::from_str(request_command), request_data.to_string(), storage);
}

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut storage: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    loop {
        let bytes_read = stream.read(&mut buffer).await.expect("Failed to read message in the buffer");
        if bytes_read == 0 {
            println!("Connection closed");
            break;
        }
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Request:\n{}", request);
        let response  = respond(request.into_owned(), &mut storage);
        stream.write(response.as_bytes()).await.expect("Failed to send response");
    }
}

#[tokio::main]
async fn main() {
    let port = "8080";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    println!("Server running on localhost at PORT={}", port);
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}