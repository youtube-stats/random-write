use chrono::prelude::DateTime;
use chrono::prelude::Local;

pub struct SubStoreDatum {
    pub time: DateTime<Local>,
    pub ids: i32,
    pub subs: i32
}