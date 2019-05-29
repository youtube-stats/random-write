extern crate actix;
extern crate actix_protobuf;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate postgres;
extern crate prost;
#[macro_use]
extern crate prost_derive;

use actix_protobuf::*;
use actix_web::*;
use actix_web::web::{Data, resource, get};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;
use postgres::Connection;

type Msg = (i32,i32);
type Payload = Vec<Msg>;
type Send = Sender<Payload>;
type Recv = Receiver<Payload>;
type Chan = (Send, Recv);

#[derive(Clone, PartialEq, Message)]
pub struct Subs {
    #[prost(int32, repeated, tag = "1")]
    pub id: Vec<i32>,
    #[prost(int32, repeated, tag = "2")]
    pub subs: Vec<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Views {
    #[prost(int32, repeated, tag = "1")]
    pub id: Vec<i32>,
    #[prost(int32, repeated, tag = "2")]
    pub views: Vec<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Videos {
    #[prost(int32, repeated, tag = "1")]
    pub id: Vec<i32>,
    #[prost(int32, repeated, tag = "2")]
    pub videos: Vec<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Metrics {
    #[prost(message, tag = "1")]
    pub subs: Option<Subs>,
    #[prost(message, tag = "2")]
    pub views: Option<Views>,
    #[prost(message, tag = "3")]
    pub videos: Option<Videos>
}

const POSTGRESQL_URL: &'static str = "postgresql://admin@localhost:5432/youtube";

/*fn sub_check(conn: &Connection, subs: &SubList) -> Vec<Msg> {

}*/

fn handler(state: Data<Receivers>, msg: ProtoBuf<Metrics>) -> HttpResponse {
    println!("model: {:?}", msg);

    let sends: Receivers = state.get_ref().clone();
    let subs_send: Send = sends.subs;
    let views_send: Send = sends.views;
    let videos_send: Send = sends.videos;

    let mut subs: Vec<Msg> = Vec::new();
    let mut views: Vec<Msg> = Vec::new();
    let mut videos: Vec<Msg> = Vec::new();
    {
        let msg = msg.clone();
        let subs_vec: Subs = msg.subs.unwrap();
        let views_vec: Views = msg.views.unwrap();
        let videos_vec: Videos = msg.videos.unwrap();

        let len: usize = subs_vec.subs.len();
        for i in 0..len {
            let id: i32 = subs_vec.id[i];
            let sub: i32 = subs_vec.subs[i];
            let value: Msg = (id, sub);

            subs.push(value);

            let id: i32 = views_vec.id[i];
            let view: i32 = views_vec.views[i];
            let value: Msg = (id, view);

            views.push(value);

            let id: i32 = videos_vec.id[i];
            let video: i32 = videos_vec.videos[i];
            let value: Msg = (id, video);

            videos.push(value);
        }
    }

    let subs: Vec<Msg> = subs;
    let views: Vec<Msg> = views;
    let videos: Vec<Msg> = videos;

    subs_send.send(subs).expect("Could not send subs");
    views_send.send(views).expect("Could not send views");
    videos_send.send(videos).expect("Could not send videos");

    HttpResponse::Ok().finish()
}

#[derive(Clone)]
struct Receivers {
    subs: Send,
    views: Send,
    videos: Send
}

fn main() {
    println!("Hello, world!");
    ::std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let sys = actix::System::new("db-writer");

    let (subs_tx, subs_rx): Chan = channel();
    let (views_tx, views_rx): Chan = channel();
    let (videos_tx, videos_rx): Chan = channel();

    thread::spawn(move || {
        let params: &'static str = POSTGRESQL_URL;
        let tls: postgres::TlsMode = postgres::TlsMode::None;

        let conn: postgres::Connection =
            postgres::Connection::connect(params, tls).unwrap();

        loop {
            println!("Waiting for subs message");
            let subs_row: Payload = subs_rx.recv().expect("Something went wrong with sub message");
            println!("Got subs msg {:?}", subs_row);
        }
    });

    thread::spawn(move || {
        let params: &'static str = POSTGRESQL_URL;
        let tls: postgres::TlsMode = postgres::TlsMode::None;

        let conn: postgres::Connection =
            postgres::Connection::connect(params, tls).unwrap();

        loop {
            println!("Waiting for views message");
            let views_row: Payload = views_rx.recv().expect("Something went wrong with views message");
            println!("Got views msg {:?}", views_row);
        }
    });

    thread::spawn(move || {
        let params: &'static str = POSTGRESQL_URL;
        let tls: postgres::TlsMode = postgres::TlsMode::None;

        let conn: postgres::Connection =
            postgres::Connection::connect(params, tls).unwrap();

        loop {
            println!("Waiting for videos message");
            let videos_row: Payload = videos_rx.recv().expect("Something went wrong with videos message");
            println!("Got videos msg {:?}", videos_row);
        }
    });

    HttpServer::new(move || App::new()
        .data(Receivers {
            subs: subs_tx.clone(),
            views: views_tx.clone(),
            videos: videos_tx.clone()
        })
        .wrap(middleware::Logger::default())
        .service(
            resource("/put")
                .route(get().to(handler)))
    ).bind("0.0.0.0:8080")
    .expect("Can not bind to port 8080")
    .start();

    println!("Started http server: 0.0.0.0:8081");
    let _ = sys.run();
}
