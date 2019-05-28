extern crate actix_web;
extern crate quick_protobuf;

pub mod metrics;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::{Data,resource,get};
//use metrics::{Subs,Views,Videos,Metrics};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;

#[derive(Clone)]
struct Receivers {
    subs: Sender<(i32,i32)>,
    views: Sender<(i32,i32)>,
    videos: Sender<(i32,i32)>
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
        .service(
            resource("/subs")
                .route(get().to(put_subs)))
        .service(
            resource("/views")
                .route(get().to(put_views)))
        .service(
            resource("/videos")
                .route(get().to(put_videos)))

    ).bind("0.0.0.0:8080")
    .expect("Can not bind to port 8080")
    .run()
     .expect("Cannot start server");
}
