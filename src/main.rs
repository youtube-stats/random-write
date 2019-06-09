extern crate actix;
extern crate actix_protobuf;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate postgres;
extern crate prost;
#[macro_use]
extern crate prost_derive;

pub mod types;
use types::{ProtoSubs,SubsStore};

pub mod statics;
use statics::{CACHE_SIZE,POSTGRESQL_URL};

pub mod lib;
use lib::{get_insert_str,handler};

use actix::System;
use actix::SystemRunner;
use actix_web::{HttpResponse, HttpServer, App, middleware};
use actix_web::web::{resource, post, Data};
use chrono::prelude::{DateTime,Local};
use postgres::{Connection,TlsMode};
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;

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
                let query: String = get_insert_str(&store);
                let query: &str = query.as_str();

                conn.execute(query, &[])
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
