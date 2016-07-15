#![allow(dead_code)]

extern crate anidb;

use std::net::UdpSocket;
use std::str;

use anidb::Result;

pub struct MockServer {
    pub socket: UdpSocket,
}

impl MockServer {
    pub fn new(port: u16) -> Result<MockServer> {
        let socket = try!(UdpSocket::bind(("0.0.0.0", port)));
        Ok(MockServer { socket: socket })
    }

    pub fn update(&self) {
        let mut buf = [0; 2048];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    println!("amt: {}", amt);
                    println!("src: {}", src);
                    println!("{}", str::from_utf8(&buf).unwrap_or(""));
                },
                Err(e) => {
                    println!("couldn't recieve a datagram: {}", e);
                }
            }
        }
    }
}

