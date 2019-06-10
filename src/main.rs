extern crate hyper;
extern crate postgres;
extern crate quick_protobuf;

pub mod types;
use types::SubStoreDatum;

pub mod statics;
use statics::{CACHE_SIZE, POSTGRESQL_URL};

pub mod message;
use message::Ack;

use hyper::{Response, Server, Body, Request};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use postgres::{Connection, TlsMode};
use std::net::SocketAddr;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;
use quick_protobuf::serialize_into_vec;
use hyper::rt::run;

pub fn main() {
    let (sx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

    thread::spawn(move || {
        let conn: Connection = {
            let params: &'static str = POSTGRESQL_URL;
            let tls: TlsMode = TlsMode::None;

            Connection::connect(params, tls)
                .expect("Could not connect to database")
        };
        let mut store: Vec<SubStoreDatum> = {
            let capacity: usize = 2 * CACHE_SIZE;
            Vec::with_capacity(capacity)
        };

        loop {
            {
                println!("Waiting for message");
                let other: Vec<u8> = rx.recv()
                    .expect("Could not retrieve message");
                println!("Got message {:?}", other);
                println!("New size of store is {}", store.len());
            }
        }
    });

    let addr: SocketAddr = ([0u8, 0u8, 0u8, 0u8], 8081u16).into();

    let f = |req: Request<Body>| {
        let mut message: Ack = Ack::default();
        message.ok = true;
        let vec: Vec<u8> = serialize_into_vec(&message)
            .expect("Cannot serialize `foobar`");
        let body: Body = Body::from(vec);
        Response::new(body)
    };

    let new_service = move || {
        service_fn_ok(f)
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(move |e| eprintln!("server error: {}", e));

    run(server);
}
