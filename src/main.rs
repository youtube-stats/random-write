extern crate bytes;
extern crate chrono;
extern crate hyper;
extern crate postgres;
extern crate quick_protobuf;

pub mod types;
use types::SubStoreDatum;

pub mod statics;
use statics::{CACHE_SIZE, POSTGRESQL_URL};

pub mod message;
use message::{Ack, SubMessage};

use crate::bytes::Bytes;
use crate::hyper::{Response, Server, Body, Request, Method};
use crate::hyper::rt::{Future, Stream, run};
use crate::hyper::service::service_fn_ok;
use crate::postgres::{Connection, TlsMode};
use ::std::net::SocketAddr;
use ::std::sync::mpsc::{Sender, Receiver};
use ::std::sync::mpsc::channel;
use ::std::thread;
use crate::quick_protobuf::{serialize_into_vec, deserialize_from_slice};
use std::sync::Arc;

pub fn main() {
    let (mut sx, mut rx): (Sender<SubMessage>, Receiver<SubMessage>) = channel();

    {
        let f = move || {
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
                    let other: SubMessage = rx.recv()
                        .expect("Could not retrieve message");
                    println!("Got message {:?}", other);


                    println!("New size of store is {}", store.len());
                }
            }
        };

        thread::spawn(f);
    }

    {
        let addr: SocketAddr = ([0u8, 0u8, 0u8, 0u8], 8082u16).into();

        let new_service = move || {
            let sx: Sender<SubMessage> = {
                let sx: &Sender<SubMessage> = &sx;
                sx.clone()
            };

            service_fn_ok(move |req: Request<Body>| {
                let method: Method = req.method().clone();
                let path: String = {
                    let path: &str = req.uri().path();

                    path.to_string().clone()
                };
                let _endpoint: String = format!("/post");

                let bytes = {
                    let body: Body = req.into_body();
                    let entire_body = body.concat2();
                    let mut bytes: Vec<u8> = Vec::new();

                    let resp = entire_body.map(|body| {
                        let other: Bytes = body.into_bytes();
                        let mut other: Vec<u8> = other.to_vec();

                        println!("Read {} bytes", other.len());
                        bytes.append(&mut other);
                    });

                    bytes
                };
                let bytes: &[u8] = bytes.as_slice();
                let msg_option = deserialize_from_slice(bytes);

                let good_resp = {
                    let mut message: Ack = Ack::default();
                    message.ok = true;
                    let vec: Vec<u8> = serialize_into_vec(&message)
                        .expect("Cannot serialize `foobar`");

                    let body: Body = Body::from(vec);
                    Response::new(body)
                };
                let bad_resp = {
                    let mut message: Ack = Ack::default();
                    message.ok = false;
                    let vec: Vec<u8> = serialize_into_vec(&message)
                        .expect("Cannot serialize `foobar`");

                    let body: Body = Body::from(vec);
                    Response::new(body)
                };

                match (method, path, msg_option) {
                    (Method::POST, _endpoint, Ok(t)) => {
                        sx.send(t)
                            .expect("Could not send msg");
                        good_resp
                    },
                    _ => bad_resp
                }
            })
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        run(server);
    }
}
