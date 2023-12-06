use super::router::Router;
use http::http_request::Request;
use std::{io::Read, net::TcpListener};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Listening on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");

            let mut buffer = [0; 2048];
            stream.read(&mut buffer).unwrap();

            let req: Request = String::from_utf8(buffer.to_vec()).unwrap().into();

            Router::route(req, &mut stream);
        }
    }
}
