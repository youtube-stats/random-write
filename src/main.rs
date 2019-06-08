extern crate actix;
extern crate actix_protobuf;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate postgres;
extern crate prost;
#[macro_use]
extern crate prost_derive;

use actix::System;
use actix::SystemRunner;
use actix_protobuf::*;
use actix_web::*;
use actix_web::web::{Data, resource, get};
use postgres::Connection;
use postgres::TlsMode;
use postgres::stmt::Statement;
use postgres::types::ToSql;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;

pub mod types {
    #[derive(Clone, PartialEq, Message)]
    pub struct Subs {
        #[prost(int32, repeated, tag = "1")]
        pub time: Vec<i32>,
        #[prost(int32, repeated, tag = "2")]
        pub id: Vec<i32>,
        #[prost(int32, repeated, tag = "3")]
        pub subs: Vec<i32>
    }

    impl Subs {
        pub fn to_store(self: &Subs) -> Vec<SubsStore> {
            let mut store: Vec<SubsStore> = Vec::new();
            let len: usize = self.subs.len();

            for i in 0..len {
                let value: SubsStore = SubsStore {
                    time: self.time[i],
                    id: self.id[i],
                    subs: self.subs[i]
                };

                store.push(value)
            }

            store
        }
    }

    pub struct SubsStore {
        pub time: i32,
        pub id: i32,
        pub subs: i32
    }
}

use types::{Subs,SubsStore};

use std::ops::Range;

pub mod statics {
    pub const POSTGRESQL_URL: &'static str = "postgresql://admin@localhost:5432/youtube";
    pub const CACHE_SIZE: usize = 500;
}

use statics::CACHE_SIZE;

pub fn handler(state: Data<Sender<Subs>>, msg: ProtoBuf<Subs>) -> HttpResponse {
    let t: Subs = msg.0;
    println!("Received model: {:?}", t);

    let sender: &Sender<Subs> = state.get_ref();
    sender.send(t).expect("Could not send protobuf message");
    HttpResponse::Ok().finish()
}

pub fn get_insert_str() -> String {
    let mut str_buffer: String = {
        let string: &'static str = "INSERT INTO youtube.stats.subs (time, id, subs) VALUES ";

        let capacity: usize = 4 * CACHE_SIZE;
        let mut str_buffer: String = String::with_capacity(capacity);
        str_buffer.push_str(string);
        str_buffer
    };

    let range: Range<usize> = 1..CACHE_SIZE;
    let step: usize = 3;

    for i in range.step_by(step) {
        let string: String = format!("(${},${},${}),", i, i + 1, i + 2);
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

        params[counter] = &sub.id;
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
    let (sx, rx): (Sender<Subs>, Receiver<Subs>) = channel();

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
                println!("Waiting for message message");
                let other: Subs = rx.recv().expect("Could not retrieve message");
                println!("Got message {:?}", other);

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
        .service(
            resource("/put")
                .route(get().to(handler)))
    ).bind("0.0.0.0:8080")
    .expect("Can not bind to port 8080")
    .workers(8)
    .start();

    println!("Started http server: 0.0.0.0:8080");
    let _ = sys.run();
}
