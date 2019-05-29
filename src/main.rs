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
use postgres::stmt::Statement;
use postgres::rows::{Rows, Row};

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

fn subs_check(conn: &Connection, subs: &Vec<Msg>) -> Payload {
    let query: &'static str = "SELECT subs FROM youtube.stats.metric_subs WHERE channel_id = $1 ORDER BY time ASC LIMIT 1";
    let stmt: Statement = conn.prepare_cached(query).unwrap();

    let mut clean: Vec<Msg> = Vec::new();
    for value in subs {
        let rows: Rows = stmt.query(&[]).expect("Could not query subs table");
        if rows.len() == 0 {
            clean.push(*value);
            continue
        }

        let row: Row = rows.iter().last().expect("could not get retrieve subs row");
        let sub: i32 = row.get(0);

        if sub != value.1 {
            clean.push(*value);
        }
    }

    clean
}

fn views_check(conn: &Connection, views: &Vec<Msg>) -> Payload {
    let query: &'static str = "SELECT subs FROM youtube.stats.metric_views WHERE channel_id = $1 ORDER BY time ASC LIMIT 1";
    let stmt: Statement = conn.prepare_cached(query).unwrap();

    let mut clean: Vec<Msg> = Vec::new();
    for value in views {
        let rows: Rows = stmt.query(&[]).expect("Could not query views table");
        if rows.len() == 0 {
            clean.push(*value);
            continue
        }

        let row: Row = rows.iter().last().expect("could not get retrieve views row");
        let view: i32 = row.get(0);

        if view != value.1 {
            clean.push(*value);
        }
    }

    clean
}

fn videos_check(conn: &Connection, videos: &Vec<Msg>) -> Payload {
    let query: &'static str = "SELECT subs FROM youtube.stats.metric_videos WHERE channel_id = $1 ORDER BY time ASC LIMIT 1";
    let stmt: Statement = conn.prepare_cached(query).unwrap();

    let mut clean: Vec<Msg> = Vec::new();
    for value in videos {
        let rows: Rows = stmt.query(&[]).expect("Could not query videos table");
        if rows.len() == 0 {
            clean.push(*value);
            continue
        }

        let row: Row = rows.iter().last().expect("could not get retrieve videos row");
        let videos: i32 = row.get(0);

        if videos != value.1 {
            clean.push(*value);
        }
    }

    clean
}


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
            let subs: Payload = subs_rx.recv().expect("Something went wrong with sub message");
            println!("Got subs msg {:?}", subs);

            let subs: Payload = subs_check(&conn, &subs);
            println!("Inserting subs {:?}", subs);
        }
    });

    thread::spawn(move || {
        let params: &'static str = POSTGRESQL_URL;
        let tls: postgres::TlsMode = postgres::TlsMode::None;

        let conn: postgres::Connection =
            postgres::Connection::connect(params, tls).unwrap();

        loop {
            println!("Waiting for views message");
            let views: Payload = views_rx.recv().expect("Something went wrong with views message");
            println!("Got views msg {:?}", views);

            let views: Payload = views_check(&conn, &views);
            println!("Inserting views {:?}", views);
        }
    });

    thread::spawn(move || {
        let params: &'static str = POSTGRESQL_URL;
        let tls: postgres::TlsMode = postgres::TlsMode::None;

        let conn: postgres::Connection =
            postgres::Connection::connect(params, tls).unwrap();

        loop {
            println!("Waiting for videos message");
            let videos: Payload = videos_rx.recv().expect("Something went wrong with videos message");
            println!("Got videos msg {:?}", videos);

            let videos: Payload = videos_check(&conn, &videos);
            println!("Inserting videos {:?}", videos);
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
    .workers(8)
    .start();

    println!("Started http server: 0.0.0.0:8081");
    let _ = sys.run();
}
