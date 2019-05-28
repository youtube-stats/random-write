extern crate actix;
extern crate actix_protobuf;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate prost;
#[macro_use]
extern crate prost_derive;

use actix_protobuf::*;
use actix_web::*;
use actix_web::web::{Data, resource, get};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;

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

fn index(msg: ProtoBuf<Metrics>) -> HttpResponse {
    println!("model: {:?}", msg);
    HttpResponse::Ok().finish()
}

#[derive(Clone)]
struct Receivers {
    subs: Sender<(i32,i32)>,
    views: Sender<(i32,i32)>,
    videos: Sender<(i32,i32)>
}

fn put(state: Data<Receivers>, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    HttpResponse::Ok().finish()
}

fn put_subs(state: Data<Receivers>, req: HttpRequest) -> HttpResponse {
    let s = state.get_ref().clone();
    let subs: Sender<(i32,i32)> = s.subs;

    format!("{:?}!", req);
    subs.send((1,2)).expect("Could not send subs msg");

    HttpResponse::Ok().finish()
}

fn put_views(state: Data<Receivers>, req: HttpRequest) -> HttpResponse {
    let s = state.get_ref().clone();
    let views: Sender<(i32,i32)> = s.views;

    format!("{:?}!", req);
    views.send((3,4)).expect("Could not send views msg");

    HttpResponse::Ok().finish()
}

fn put_videos(state: Data<Receivers>, req: HttpRequest) -> HttpResponse {
    let s = state.get_ref().clone();
    let videos: Sender<(i32,i32)> = s.videos;

    format!("{:?}!", req);
    videos.send((5,6)).expect("Could not send videos msg");

    HttpResponse::Ok().finish()
}

fn main() {
    println!("Hello, world!");
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("prost-example");

    let (mut subs_tx, mut subs_rx): (Sender<(i32,i32)>, Receiver<(i32,i32)>) = channel();
    let (mut views_tx, mut views_rx): (Sender<(i32,i32)>, Receiver<(i32,i32)>) = channel();
    let (mut videos_tx, mut videos_rx): (Sender<(i32,i32)>, Receiver<(i32,i32)>) = channel();

    thread::spawn(move || {
        loop {
            println!("Waiting for subs message");
            let subs_row: (i32, i32) = subs_rx.recv().expect("Something went wrong with sub message");
            println!("Got subs msg {} {}", subs_row.0, subs_row.1);
        }
    });

    thread::spawn(move || {
        loop {
            println!("Waiting for views message");
            let views_row: (i32, i32) = views_rx.recv().expect("Something went wrong with views message");
            println!("Got views msg {} {}", views_row.0, views_row.1);
        }
    });

    thread::spawn(move || {
        loop {
            println!("Waiting for videos message");
            let videos_row: (i32, i32) = videos_rx.recv().expect("Something went wrong with videos message");
            println!("Got videos msg {} {}", videos_row.0, videos_row.1);
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
                .route(get().to(put)))
    ).bind("0.0.0.0:8080")
    .expect("Can not bind to port 8080")
    .run()
     .expect("Cannot start server");
}
