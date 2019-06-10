extern crate chrono;
extern crate hyper;
extern crate postgres;
extern crate quick_protobuf;
extern crate rand;

pub mod statics;
use statics::{CACHE_SIZE, POSTGRESQL_URL, INSERT};

pub mod message;
use message::ChannelMessage;

use crate::bytes::Bytes;
use crate::hyper::{Response, Server, Body, Request, Method};
use crate::hyper::rt::{Future, Stream, run};
use crate::hyper::service::service_fn_ok;
use crate::postgres::{Connection, TlsMode};
use crate::quick_protobuf::{serialize_into_vec, deserialize_from_slice};
use ::std::net::SocketAddr;
use std::ops::Range;

pub fn main() {
    {
        let addr: SocketAddr = ([0u8, 0u8, 0u8, 0u8], 8082u16).into();

        let new_service = move || {
            service_fn_ok(move |req: Request<Body>| {

            }
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        run(server);
    }
}
