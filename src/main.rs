extern crate quick_protobuf;

pub mod message;
use message::{SubMessage, ChannelRowMessage};

use quick_protobuf::{deserialize_from_slice, serialize_into_vec};
use std::collections::HashMap;
use std::net::{TcpStream, Ipv4Addr, IpAddr, SocketAddr};
use std::io::Read;
use std::process::exit;

static KEY_PORT: &'static u16 = &3333u16;
static WRITE_PORT: &'static u16 = &3334u16;
static QUERY_PORT: &'static u16 = &3335u16;
const KEY_SIZE_TOP: usize = 39;

pub fn call(port: &u16) -> TcpStream {
    let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    println!("Calling service on port {}", port);

    let addr: SocketAddr = SocketAddr::new(ip, *port);
    TcpStream::connect(&addr)
        .expect("unable to connect to TCP server")
}

pub fn get_key() -> String {
    let mut stream: TcpStream = call(KEY_PORT);

    let mut buf: [u8; KEY_SIZE_TOP] = [0u8; KEY_SIZE_TOP];
    let n = stream.read(&mut buf)
        .expect("Could not read key");

    let vec: Vec<u8> = buf.to_vec();
    let key: String = std::string::String::from_utf8(vec)
        .expect("Could not convert key to string");

    println!("Got key {}", key);
    key
}


pub fn main() {
    println!("Starting random service");
    let num: String = std::env::args().last().unwrap();
    println!("Will query with path {}", num);

    loop {
        let key: String = get_key();
    }
}
