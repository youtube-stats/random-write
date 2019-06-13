extern crate byteorder;
extern crate quick_protobuf;
extern crate ureq;

pub mod message;
use message::{SubMessage, ChannelRowMessage};

use byteorder::{WriteBytesExt, LittleEndian};
use quick_protobuf::{deserialize_from_slice, serialize_into_vec};
use std::collections::HashMap;
use std::net::{TcpStream, Ipv4Addr, IpAddr, SocketAddr};
use std::io::Read;
use std::process::exit;

#[derive(Clone, Debug)]
pub struct ChannelRow {
    pub id: i32,
    pub serial: String
}

static KEY_PORT: &'static u16 = &3333u16;
static QUERY_PORT: &'static u16 = &3334u16;
static WRITE_PORT: &'static u16 = &3335u16;
const KEY_SIZE_TOP: usize = 39;
const QUERY_BUFFER_SIZE: usize = 2000;

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
    let n: usize = stream.read(&mut buf)
        .expect("Could not read key");

    println!("Received {} bytes", n);

    let vec: Vec<u8> = buf[..n].to_vec();
    let key: String = std::string::String::from_utf8(vec)
        .expect("Could not convert key to string");

    println!("Got key {}", key);
    key
}

pub fn get_channels(key: &String, n: u32) {
    let mut stream: TcpStream = call(QUERY_PORT);
    stream.write_u32::<LittleEndian>(n)
        .expect("Could not write u32 to socket");

    let mut buf: [u8; QUERY_BUFFER_SIZE] = [0u8; QUERY_BUFFER_SIZE];
    let n: usize = stream.read(&mut buf)
        .expect("Could not read query results");

    println!("Received {} bytes", n);
    let bytes: &[u8] = &buf[..n];
    let msg: ChannelRowMessage = deserialize_from_slice(bytes)
        .expect("Could not parse query results");

    println!("Received results {:?}", msg);
}

pub fn main() {
    println!("Starting random service");
    let num: String = std::env::args().last().unwrap();
    println!("Limit query to top {} results", num);
    let n: u32 = num.parse::<u32>()
        .expect("Could not parse cmd arg to int");

    loop {
        let key: String = get_key();
        get_channels(&key, n);
    }
}
