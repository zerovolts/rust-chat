use std::env;

mod server;
use server::Server;

mod client;
use client::Client;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 && args[1] == "server" {
        println!("Starting server...");
        Server::new("127.0.0.1:8080").start()
    } else {
        println!("Starting client...");
        Client::new("127.0.0.1:8080").start()
    }
}
