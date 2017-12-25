use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::{thread, time};

pub struct Client {
    address: String,
    stream: TcpStream
}

impl Client {
    pub fn new(address: &str) -> Client {
        Client {
            address: String::from(address),
            stream: TcpStream::connect(address)
                .expect("Couldn't connect to the server.")
        }
    }

    pub fn start(&mut self) {
        self.send("hello world!");
    }

    pub fn send(&mut self, message: &str) {
        self.stream.write(String::from(message).as_bytes());
    }
}
