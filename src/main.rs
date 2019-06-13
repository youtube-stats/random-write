extern crate quick_protobuf;

pub mod message;
use message::{SubMessage, ChannelRowMessage};

use quick_protobuf::{deserialize_from_slice, serialize_into_vec};
use std::collections::HashMap;

pub const KEY_URL: &'static str = "127.0.0.1:3333";
pub const WRITE_URL: &'static str = "127.0.0.1:3334";
pub const QUERY_URL: &'static str = "127.0.0.1:3335";

pub fn main() {
    println!("Starting random service");
    let num: String = std::env::args().last().unwrap();
    println!("Will query with path {}", num);
}
