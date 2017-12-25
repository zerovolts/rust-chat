use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};

pub struct Server {
    address: String,
    listener: TcpListener,
    connections: Vec<Arc<Mutex<TcpStream>>>,
    messages: Vec<String>
}

impl Server {
    pub fn new(address: &str) -> Server {
        Server {
            address: String::from(address),
            listener: TcpListener::bind(address).unwrap(),
            connections: Vec::new(),
            messages: Vec::new()
        }
    }

    pub fn start(&mut self) {
        println!("Now listening on port 8080");

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut stream = Arc::new(Mutex::new(stream));
                    self.connections.push(stream.clone());
                    Server::handle_client(stream);
                }
                Err(e) => ()
            }
        }
    }

    fn handle_client(mut stream: Arc<Mutex<TcpStream>>) {
        let mut buffer = String::new();
        stream.lock().unwrap().read_to_string(&mut buffer);
        println!("{}", buffer);
    }
}
