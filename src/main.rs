extern crate quick_protobuf;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod statics;
use statics::{QUERY_URL, KEY_URL, WRITE_URL};

pub mod message;
use message::{SubMessage, ChannelMessage, Ack};

use quick_protobuf::{deserialize_from_slice, serialize_into_vec};
use std::collections::HashMap;
use rust_random_write::YoutubeResponseType;
use reqwest::Client;
use std::io::Read;

pub fn main() {
    println!("Starting random service");
    let client: Client = reqwest::Client::new();

    loop {
        let bytes: Vec<u8> = {
            let url: &'static str = QUERY_URL;
            let mut buffer: Vec<u8> = Vec::new();
            let bytes = reqwest::get(url)
                .expect("Could not get response")
                .bytes();

            for b in bytes {
                let value: u8 = b.expect("Could not get byte");
                buffer.push(value);
            }

            buffer
        };
        let bytes: &[u8] = bytes.as_slice();

        println!("Received bytes: {:?}", bytes);

        let msg: ChannelMessage = {
            let msg: ChannelMessage = deserialize_from_slice(bytes)
                .expect("Could not deserialize channel body");

            println!("Got message {:?}", msg);
            msg
        };

        let store: HashMap<String, i32> = {
            let mut store: HashMap<String, i32> = HashMap::new();

            for i in 0..50 {
                let k: String = msg.serials[i].to_string();
                let v: i32 = msg.ids[i];

                store.insert(k, v);
            }

            store
        };
        let metrics: YoutubeResponseType = {
            #[allow(unused_assignments)]
            let mut json: Option<YoutubeResponseType> = None;

            let keys: Vec<String> = {
                let mut keys: Vec<String> = Vec::new();

                for k in store.keys() {
                    let value: String = k.clone();
                    keys.push(value);
                }

                keys
            };
            let key: String = {
                let url: &'static str = KEY_URL;
                let key: String = reqwest::get(url)
                    .expect("Could not get response")
                    .text()
                    .expect("Could not get body");

                println!("Got key {}", key);
                key
            };
            let keys_str: String = keys.join(",");

            loop {
                let url: String =
                    format!("https://www.googleapis.com/youtube/v3/channels?part=statistics&key={}&id={}",
                            key, keys_str);
                let url: &str = url.as_str();

                let s: String = reqwest::get(url)
                    .expect("Could not query google api").text()
                    .expect("Could not retrieve json body");
                let s: &str = s.as_str();

                let json_obj_result = serde_json::from_str(s);

                if json_obj_result.is_err() {
                    eprintln!("Failed to parse json - repeating API call");
                    continue;
                }

                let json_obj: YoutubeResponseType = json_obj_result.unwrap();
                json = Some(json_obj);
                break;
            }

            json.unwrap()
        };

        let ack_msg: Ack = {
            let message: SubMessage = {
                let mut write_msg: SubMessage = SubMessage::default();
                let mut ids: Vec<i32> = Vec::new();
                let mut subs: Vec<i32> = Vec::new();

                println!("Got {} metrics", metrics.items.len());
                for item in metrics.items {
                    let k: &str = item.id.as_str();

                    let value: i32 = store.get(k)
                        .expect("Could not find key").clone();

                    ids.push(value);
                    let value: i32 = item.statistics.subscriberCount.parse::<i32>()
                        .expect("Could not parse sub string");

                    subs.push(value);
                }

                write_msg.ids = ids;
                write_msg.subs = subs;

                write_msg
            };

            println!("Sending message to write server {:?}", message);

            let write_data: Vec<u8> = serialize_into_vec(&message)
                .expect("Could not serialize write message");

            println!("Sending binary to write server: {:?}", write_data);

            let url: &'static str = WRITE_URL;
            let write_resp_text: String = client.post(url).body(write_data).send()
                .expect("Could not reach write server").text()
                .expect("Could not retrieve response");

            let write_resp_bytes: &[u8] = write_resp_text.as_bytes();

            let ack_msg: Ack = deserialize_from_slice(write_resp_bytes)
                .expect("Could not deserialize write ack");

            ack_msg
        };

        if ack_msg.ok {
            println!("Write server transfer success")
        }

        println!("Restarting loop");
    }
}
