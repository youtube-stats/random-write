use prost::Message;
use chrono::prelude::DateTime;
use chrono::prelude::Local;

#[derive(Clone, PartialEq, Message)]
pub struct Acknowledge {
    #[prost(bool, tag = "1")]
    pub ok: Option<bool>
}

#[derive(Clone, PartialEq, Message)]
pub struct ProtoSubs {
    #[prost(uint32, repeated, tag = "1")]
    pub time: Vec<u32>,
    #[prost(int32, repeated, tag = "2")]
    pub id: Vec<i32>,
    #[prost(int32, repeated, tag = "3")]
    pub subs: Vec<i32>
}

impl ProtoSubs {
    pub fn store(self: &ProtoSubs) -> SubStore {
        let mut store: SubStoreDatum = Vec::new();

        for item in &self.items {
            let time: DateTime<Local> = Local::now();

            let ids: i32 = item.id;
            let subs: i32 = item.sub;

            let sub_store: SubStoreDatum = SubStoreDatum {
                time,
                ids,
                subs
            };

            store.cache.push(sub_store);
        }

        store
    }
}

pub struct SubStoreDatum {
    pub time: DateTime<Local>,
    pub ids: i32,
    pub subs: i32
}

pub struct SubStore {
    pub cache: Vec<SubStoreDatum>
}