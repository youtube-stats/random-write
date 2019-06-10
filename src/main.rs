extern crate bytes;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate postgres;
extern crate quick_protobuf;

pub mod types;
use types::SubStoreDatum;

pub mod statics;
use statics::{CACHE_SIZE, POSTGRESQL_URL};

pub mod message;
use message::{Ack, SubMessage};

use crate::futures::future::{Either, ok};
use crate::futures::Stream;
use crate::hyper::{Response, Server, Body, Request, Method};
use crate::hyper::service::service_fn;
use crate::hyper::rt::{Future,run};
use crate::postgres::{Connection, TlsMode};
use ::std::net::SocketAddr;
use ::std::sync::mpsc::{Sender, Receiver};
use ::std::sync::mpsc::channel;
use ::std::thread;
use crate::quick_protobuf::{serialize_into_vec, deserialize_from_slice};
use hyper::service::service_fn_ok;
use bytes::Bytes;

fn echo(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = hyper::Error> {
    let (parts, body) = req.into_parts();

    match (parts.method, parts.uri.path()) {
        (Method::POST, "/") => {
            let entire_body = body.concat2();
            let resp = entire_body.map(|body| {
                println!("Size of body {}", body.len());
                let body = Body::from(format!("Read {} bytes", body.len()));
                Response::new(body)
            });
            Either::A(resp)
        }
        _ => {
            let body = Body::from("Can only POST to /");
            let resp = ok(Response::new(body));
            Either::B(resp)
        }
    }
}

pub fn main() {
    let (sx, rx): (Sender<SubMessage>, Receiver<SubMessage>) = channel();

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
        let f = |req: Request<Body>| {
            let method: Method = req.method().clone();
            let path: String = {
                let path: &str = req.uri().path();

                path.to_string().clone()
            };
            let endpoint: String = format!("/post");

            let bytes = {
                let body: Body = req.into_body();
                let entire_body = body.concat2();
                let mut bytes: Vec<u8> = Vec::new();

                let resp = entire_body.map(|body| {
                    let mut other: Bytes = body.into_bytes();
                    let mut other: Vec<u8> = other.to_vec();

                    println!("Read {} bytes", other.len());
                    bytes.append(&mut other);
                });

                bytes
            };

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

            match (method, path) {
                (Method::POST, endpoint) => good_resp,
                _                        => bad_resp
            }
        };
        let new_service = move || {
            service_fn_ok(f)
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(move |e| eprintln!("server error: {}", e));

        run(server);
    }
}
