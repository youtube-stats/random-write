extern crate byteorder;
extern crate quick_protobuf;
extern crate ureq;

pub mod message;
use message::{SubMessage, ChannelRowMessage};

use byteorder::{WriteBytesExt, LittleEndian};
use quick_protobuf::{deserialize_from_slice, serialize_into_vec};
use std::collections::HashMap;
use std::io::Read;
use std::net::{TcpStream, Ipv4Addr, IpAddr, SocketAddr};
use ureq::SerdeValue;

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

pub fn get_metrics(key: String, msg: ChannelRowMessage) -> SubMessage {
    let mut hash: HashMap<&str, i32> = HashMap::new();
    for i in 0..msg.ids.len() {
        let k: &str = &msg.serials[i];
        let v: i32 = msg.ids[i];

        hash.insert(k, v);
    }

    let mut ids: String = String::new();
    ids.push_str(&msg.serials[0]);

    let comma: &str = ",";
    for i in 1..50 {
        ids.push_str(comma);
        let string: &str = &msg.serials[i];
        ids.push_str(string);
    }

    let url: String =
        format!("https://www.googleapis.com/youtube/v3/channels?part=statistics&key={}&id={}", key, ids);
    let path: &str = url.as_str();

    let resp_option = ureq::get(path).call().into_json();
    if resp_option.is_err() {
        eprintln!("Error trying to retrieve json - calling again with different key");
        return get_metrics(get_key(), msg);
    }

    let value: SerdeValue = resp_option.unwrap();
    println!("Received response {:?}", value);

    let mut sub_msg: SubMessage = SubMessage::default();

    let items_option: Option<&Vec<SerdeValue>> = value["items"].as_array();
    if items_option.is_none() {
        eprintln!("Could not find items in json");
        return get_metrics(get_key(), msg);
    }

    let items = items_option.unwrap();
    for item in items {
        let id_option: Option<&str> = item["id"].as_str();
        if id_option.is_none() {
            eprintln!("Could not find item id");
            return get_metrics(get_key(), msg);
        }

        let id: &str = id_option.unwrap();
        let id: i32 = *hash.get(id)
            .expect("Could not find key");

        let sub_option: Option<&str> = item["statistics"]["subscriberCount"].as_str();
        if sub_option.is_none() {
            eprintln!("Could not get subscriberCount");
            return get_metrics(get_key(), msg);
        }

        let sub: &str = sub_option.unwrap();
        let sub: i32 = sub.parse::<i32>().expect("could not convert string to i32");

        sub_msg.ids.push(id);
        sub_msg.subs.push(sub);
    }

    println!("Created subscriber message object {:?}", sub_msg);
    sub_msg
}

pub fn get_channels(key: String, n: u32) -> SubMessage {
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

    let value: SubMessage = get_metrics(key, msg);
    value
}

pub fn main() {
    println!("Starting random service");
    let num: String = std::env::args().last().unwrap();
    println!("Limit query to top {} results", num);
    let n: u32 = num.parse::<u32>()
        .expect("Could not parse cmd arg to int");

    loop {
        let key: String = get_key();
        let msg: SubMessage = get_channels(key, n);
    }
}
