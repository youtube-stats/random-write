extern crate quick_protobuf;
extern crate reqwest;

pub mod statics;
use statics::URL;

pub mod message;
use message::ChannelMessage;
use quick_protobuf::deserialize_from_slice;
use std::collections::HashMap;

pub fn main() {
    println!("Starting random service");

    loop {
        let url: &'static str = URL;
        let bytes: &[u8] = reqwest::get(url)
            .expect("Could not get response")
            .text()
            .expect("Could not get body")
            .as_bytes();

        let msg: ChannelMessage = deserialize_from_slice(bytes)
            .expect("Could not deserialize body");

        println!("Got message {:?}", msg);
        let store: HashMap<String, i32> = {
            let mut store: HashMap<String, i32> = HashMap::new();

            for i in 0..50 {
                let k: String = msg.serials[i].to_string();
                let v: i32 = msg.ids[i];

                store.insert(k, v)
                    .expect("Could not insert into hash");
            }

            store
        };

        let keys = store.keys().map(|s| s).collect();
    }
}
