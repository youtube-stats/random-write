use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use postgres::{Connection, TlsMode};
use postgres::rows::Rows;

pub mod message;
use message::{ChannelMessage, IdsMessage, SerialsMessage};

pub mod statics;
use statics::{POSTGRESQL_URL,QUERY};
use quick_protobuf::serialize_into_vec;

#[derive(Clone)]
pub struct ChannelRow {
    pub id: i32,
    pub serial: [u8; 24]
}

pub struct Channels {
    pub rows: Vec<ChannelRow>
}

impl Channels {
    pub fn init() -> Channels {
        let conn: Connection = {
            let params: &'static str = POSTGRESQL_URL;
            let tls: TlsMode = TlsMode::None;

            Connection::connect(params, tls)
                .expect("Could not connect to database")
        };
        let query: &'static str = QUERY;

        let results: Rows = conn.query(query, &[])
            .expect("Could not query db");

        let mut rows: Vec<ChannelRow> = Vec::new();
        for row in &results {
            let id: i32 = row.get(0);
            let serial: String = row.get(1);
            let serial: [u8; 24] = {
                let chars: &[u8] = serial.as_bytes();
                let mut serial: [u8; 24] = [0u8; 24];
                for i in 0..24 {
                    serial[i] = chars[i];
                }

                serial
            };

            let value: ChannelRow = ChannelRow {
                id,
                serial
            };

            rows.push(value);
        }

        println!("Retrieved {} rows", rows.len());

        Channels {
            rows
        }
    }

    fn get_50(self: &Channels, rng: &ThreadRng) -> Channels {
        let mut rng: ThreadRng = rng.clone();
        let amount: usize = 50;

        let rows: Vec<ChannelRow> =
            self.rows.choose_multiple(&mut rng, amount).cloned().collect();

        Channels {
            rows
        }
    }

    pub fn get_msg(self: &Channels, rng: &ThreadRng) -> Vec<u8> {
        let sampled: Channels = self.get_50(rng);

        let mut msg: ChannelMessage = ChannelMessage::default();
        let mut ids: IdsMessage = msg.ids.expect("Could not get ids");
        let mut serials: SerialsMessage = msg.serials.expect("Could not get serials");

        for i in 0..50 {
            ids.
        }

        let bytes: Vec<u8> = serialize_into_vec(&msg)
            .expect("Could not serialize");

        bytes
    }
}