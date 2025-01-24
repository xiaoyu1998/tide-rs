use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::env;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: tide listen | tide start");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "listen" => start_server().await,
        "start" => send_command_to_server().await,
        _ => {
            eprintln!("Usage: tide listen | tide start");
            std::process::exit(1);
        }
    }
}

async fn start_server() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Tide bot is listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(socket: TcpStream) -> tokio::io::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    //writer.write_all(b"Welcome to Tide Bot server. Type 'help' for commands.\n").await?;

    while reader.read_line(&mut buffer).await? > 0 {
        let command = buffer.trim().to_string();
        println!("Received command: {}", command);

        let response = match command.as_str() {
            "help" => "Available commands: help, status, start, stop, quit\n".to_string(),
            "status" => "Bot is running.\n".to_string(),
            "start" => "Market Maker started.\n".to_string(),
            "stop" => "Market Maker stopped.\n".to_string(),
            "quit" => "Goodbye!\n".to_string(),
            _ => "Unknown command. Type 'help' for available commands.\n".to_string(),
        };
        //println!("response: {}", response);
        writer.write_all(response.as_bytes()).await?;
        buffer.clear();
        //println!("response: {}", response);

        if command == "quit" {
            break;
        }
    }

    Ok(())
}

async fn send_command_to_server() -> tokio::io::Result<()> {
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            println!("Connected to the server.");
            let message = "start\n";
            stream.write_all(message.as_bytes()).await?;
            println!("Command sent: {}", message.trim());

            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response).await?;
            println!("Server response: {}", response.trim());
        }
        Err(e) => {
            eprintln!("Failed to connect to the server: {}", e);
        }
    }

    Ok(())
}
