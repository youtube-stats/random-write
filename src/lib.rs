use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use postgres::{Connection, TlsMode};
use postgres::rows::Rows;

pub mod message;
use message::{ChannelMessage, IdsMessage, SerialMessage, SerialsMessage};

pub mod statics;
use statics::{POSTGRESQL_URL,QUERY};

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

    pub fn get_msg(self: &Channels, rng: &ThreadRng) -> ChannelMessage {
        let sampled: Channels = self.get_50(rng);

        let mut msg: ChannelMessage = ChannelMessage::default();
        let mut ids: IdsMessage = msg.ids.expect("Could not get ids");

        {
            ids.id_1 = sampled.rows[1].id;
            ids.id_2 = sampled.rows[2].id;
            ids.id_3 = sampled.rows[3].id;
            ids.id_4 = sampled.rows[4].id;
            ids.id_5 = sampled.rows[5].id;
            ids.id_6 = sampled.rows[6].id;
            ids.id_7 = sampled.rows[7].id;
            ids.id_8 = sampled.rows[8].id;
            ids.id_9 = sampled.rows[9].id;
            ids.id_10 = sampled.rows[10].id;
            ids.id_11 = sampled.rows[11].id;
            ids.id_12 = sampled.rows[12].id;
            ids.id_13 = sampled.rows[13].id;
            ids.id_14 = sampled.rows[14].id;
            ids.id_15 = sampled.rows[15].id;
            ids.id_16 = sampled.rows[16].id;
            ids.id_17 = sampled.rows[17].id;
            ids.id_18 = sampled.rows[18].id;
            ids.id_19 = sampled.rows[19].id;
            ids.id_20 = sampled.rows[20].id;
            ids.id_21 = sampled.rows[21].id;
            ids.id_22 = sampled.rows[22].id;
            ids.id_23 = sampled.rows[23].id;
            ids.id_24 = sampled.rows[24].id;
            ids.id_25 = sampled.rows[25].id;
            ids.id_26 = sampled.rows[26].id;
            ids.id_27 = sampled.rows[27].id;
            ids.id_28 = sampled.rows[28].id;
            ids.id_29 = sampled.rows[29].id;
            ids.id_30 = sampled.rows[30].id;
            ids.id_31 = sampled.rows[31].id;
            ids.id_32 = sampled.rows[32].id;
            ids.id_33 = sampled.rows[33].id;
            ids.id_34 = sampled.rows[34].id;
            ids.id_35 = sampled.rows[35].id;
            ids.id_36 = sampled.rows[36].id;
            ids.id_37 = sampled.rows[37].id;
            ids.id_38 = sampled.rows[38].id;
            ids.id_39 = sampled.rows[39].id;
            ids.id_40 = sampled.rows[40].id;
            ids.id_41 = sampled.rows[41].id;
            ids.id_42 = sampled.rows[42].id;
            ids.id_43 = sampled.rows[43].id;
            ids.id_44 = sampled.rows[44].id;
            ids.id_45 = sampled.rows[45].id;
            ids.id_46 = sampled.rows[46].id;
            ids.id_47 = sampled.rows[47].id;
            ids.id_48 = sampled.rows[48].id;
            ids.id_49 = sampled.rows[49].id;
        }

        let mut serials

        msg
    }
}