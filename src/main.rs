use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::{thread, time, env};

fn handle_client(mut stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = String::new();
    stream.lock().unwrap().read_to_string(&mut buffer);
    println!("{}", buffer);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        println!("Starting server...");
        server();
    } else {
        println!("Starting client...");
        client();
    }
}

fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut connections: Vec<Arc<Mutex<TcpStream>>> = Vec::new();
    let mut messages: Vec<String> = Vec::new();
    println!("Now listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut stream = Arc::new(Mutex::new(stream));
                connections.push(stream.clone());
                handle_client(stream);
            }
            Err(e) => ()
        }
    }
}

fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Couldn't connect to the server.");

    let message = String::from("hello there!");
    let _ = stream.write(message.as_bytes());
    println!("sent");
}
