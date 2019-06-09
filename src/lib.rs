use actix_protobuf::ProtoBuf;
use actix_web::HttpResponse;
use actix_web::web::Data;
use std::ops::Range;
use std::sync::mpsc::Sender;

pub mod types;
use types::{SubStore,SubStoreDatum,ProtoSubs};

pub mod statics;
use statics::CACHE_SIZE;

pub fn handler(state: Data<Sender<ProtoSubs>>, msg: ProtoBuf<ProtoSubs>) -> HttpResponse {
    let sender: &Sender<ProtoSubs> = state.get_ref();
    sender.send(msg.0).expect("Could not send protobuf message");

    HttpResponse::Ok().finish()
}

pub fn get_insert_str(store: &SubStore) -> String {
    let mut str_buffer: String = {
        let first: &SubStoreDatum = store.first().expect("Store is empty");

        let string: String =
            format!("INSERT INTO youtube.stats.subs (time, id, subs) VALUES ('{}',{},{})",
                    first.time,
                    first.ids,
                    first.subs);

        let string: &str = string.as_ref();

        let capacity: usize = 4 * CACHE_SIZE;
        let mut str_buffer: String = String::with_capacity(capacity);
        str_buffer.push_str(string);
        str_buffer
    };

    let range: Range<usize> = 4..CACHE_SIZE;
    let step: usize = 3;

    for i in range.step_by(step) {
        let item: &SubStoreDatum = &store[i];

        let string: String =
            format!(",('{}',{},{})", item.time, item.ids, item.subs);
        let string: &str = &string.as_str();

        str_buffer.push_str(string);
    }

    str_buffer
}