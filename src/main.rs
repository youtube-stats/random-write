extern crate quick_protobuf;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod statics;
use statics::{QUERY_URL, KEY_URL, WRITE_URL};

pub mod message;
use message::ChannelMessage;
use quick_protobuf::deserialize_from_slice;
use std::collections::HashMap;
use rust_random_write::YoutubeResponseType;
use std::error::Error;

pub fn main() {
    println!("Starting random service");

    loop {
        let msg: ChannelMessage = {
            let bytes: &[u8] = {
                let url: &'static str = QUERY_URL;
                let bytes: &[u8] = reqwest::get(url)
                    .expect("Could not get response")
                    .text()
                    .expect("Could not get body")
                    .as_bytes();

                bytes
            };

            let msg: ChannelMessage = deserialize_from_slice(bytes)
                .expect("Could not deserialize body");

            println!("Got message {:?}", msg);
            msg
        };

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
        let keys: Vec<String> = {
            let mut keys: Vec<String> = Vec::new();

            for k in store.keys() {
                let value: String = k.clone();
                keys.push(value);
            }

            keys
        };

        let key: String = {
            let url: &'static str = QUERY_URL;
            let key: String = reqwest::get(url)
                .expect("Could not get response")
                .text()
                .expect("Could not get body");

            println!("Got key {}", key);
            key
        };

        let keys_str: String = keys.join(",");

        let json_obj: YoutubeResponseType = {
            let mut json: Option<YoutubeResponseType> = None;

            loop {
                let url: String =
                    format!("https://www.googleapis.com/youtube/v3/channels?part=statistics&key={}&id={}",
                            key, keys_str);

                let s: String = reqwest::get(url)
                    .expect("Could not query google api").text()
                    .expect("Could not retrieve json body");
                let s: &str = s.as_str();

                let json_obj_result: Result<YoutubeResponseType, Error> =
                    serde_json::from_str(s);

                if json_obj_result.is_err() {
                    let err: Error = json_obj_result.err().unwrap();
                    eprintln!("Failed to parse json - repeating API call: {}", err);
                    continue;
                }

                let json_obj: YoutubeResponseType = json_obj_result.unwrap();
                json = Some(json_obj);
                break;
            }

            json.unwrap()
        };
    }
}
