extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate postgres;
extern crate serde;
extern crate serde_json;

use actix::System;
use actix::SystemRunner;
use actix_web::{HttpResponse, HttpServer, App, middleware};
use postgres::Connection;
use postgres::TlsMode;
use postgres::stmt::Statement;
use postgres::types::ToSql;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;


pub mod types {
    pub struct SubsStore {
        pub time: i32,
        pub ids: i32,
        pub subs: i32
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SlimSub {
    id: i32,
    sub: i32
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YoutubeSlim {
    items: Vec<SlimSub>
}

impl YoutubeSlim {
    pub fn to_store(self: &YoutubeSlim) -> Vec<SubsStore> {
        let mut store: Vec<SubsStore> = Vec::new();

        for item in &self.items {
            let time: u64 = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Could not get timestamp")
                .as_secs();
            let time: i32 = time as i32;

            let ids: i32 = item.id;
            let subs: i32 = item.sub;

            let sub_store: SubsStore = SubsStore {
                time,
                ids,
                subs
            };

            store.push(sub_store);
        }

        store
    }
}

use types::SubsStore;

pub mod statics {
    pub const POSTGRESQL_URL: &'static str = "postgresql://admin@localhost:5432/youtube";
    pub const CACHE_SIZE: usize = 500;
}

use statics::CACHE_SIZE;
use actix_web::web::{Json, resource, post, Data};
use std::time::SystemTime;

pub fn handler(item: Json<YoutubeSlim>, state: Data<Sender<YoutubeSlim>>) -> HttpResponse {
    let sender: &Sender<YoutubeSlim> = state.get_ref();
    sender.send(item.0).expect("Could not send protobuf message");

    HttpResponse::Ok().finish()
}

pub fn get_insert_str() -> String {
    let mut str_buffer: String = {
        let string: &'static str =
            "INSERT INTO youtube.stats.subs (time, id, subs) VALUES ($1,$2,$3)";

        let capacity: usize = 4 * CACHE_SIZE;
        let mut str_buffer: String = String::with_capacity(capacity);
        str_buffer.push_str(string);
        str_buffer
    };

    let range: Range<usize> = 4..CACHE_SIZE;
    let step: usize = 3;

    for i in range.step_by(step) {
        let string: String = format!(",(${},${},${})", i, i + 1, i + 2);
        let string: &str = &string.as_str();

        str_buffer.push_str(string);
    }

    str_buffer
}

pub fn get_insert_params(store: &Vec<SubsStore>) -> [&ToSql; CACHE_SIZE * 3] {
    let mut params: [&ToSql; CACHE_SIZE * 3] = [&0; CACHE_SIZE * 3];
    let mut counter: usize = 0;

    for sub in store {
        params[counter] = &sub.time;
        counter += 1;

        params[counter] = &sub.ids;
        counter += 1;

        params[counter] = &sub.subs;
        counter += 1;
    }

    params
}

pub fn main() {
    let sys: SystemRunner = {
        println!("Hello, world!");
        ::std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
        env_logger::init();
        System::new("db-writer")
    };
    let (sx, rx): (Sender<YoutubeSlim>, Receiver<YoutubeSlim>) = channel();

    thread::spawn(move || {
        let conn: Connection = {
            let params: &'static str = statics::POSTGRESQL_URL;
            let tls: TlsMode = TlsMode::None;

            Connection::connect(params, tls).expect("Could not connect to database")
        };
        let query: Statement = {
            let query: String = get_insert_str();
            let query: &str = query.as_str();

            conn.prepare_cached(query)
                .expect("Could not create prepared statement")
        };

        let mut store: Vec<SubsStore> = {
            let capacity: usize = 2 * CACHE_SIZE;
            Vec::with_capacity(capacity)
        };

        loop {
            {
                println!("Waiting for message");
                let other: YoutubeSlim = rx.recv().expect("Could not retrieve message");
                println!("Got message {:?}", serde_json::to_string(&other).expect("Could not serialize"));

                {
                    let mut other: Vec<SubsStore> = other.to_store();
                    store.append(&mut other);
                }
            }

            if store.len() >= CACHE_SIZE {
                println!("Inserting {} entries", CACHE_SIZE);
                let params: [&ToSql; CACHE_SIZE * 3] = get_insert_params(&store);
                query.execute(&params[..])
                    .expect("Could not insert values");

                let range: Range<usize> = 0..CACHE_SIZE;
                store.drain(range);
            }
        }
    });

    HttpServer::new(move || App::new()
        .data(sx.clone())
        .wrap(middleware::Logger::default())
        .service(resource("/post").route(post().to(handler)))
    ).bind("0.0.0.0:8081")
    .expect("Can not bind to port 8081")
    .workers(8)
    .start();

    let _ = sys.run();
}
