use std::net::TcpStream;

pub mod utils;

pub struct User{
    client: TcpStream,
    address: u16,
}

