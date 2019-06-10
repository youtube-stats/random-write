use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

#[derive(Clone)]
pub struct ChannelRow {
    pub id: i32,
    pub serial: [char; 24]
}

pub struct Channels {
    pub rows: Vec<ChannelRow>
}

impl Channels {
    pub fn get_50(self: &Channels, rng: &ThreadRng) -> Channels {
        let mut rng: ThreadRng = rng.clone();
        let amount: usize = 50;

        let rows: Vec<ChannelRow> =
            self.rows.choose_multiple(&mut rng, amount).cloned().collect();

        Channels {
            rows
        }
    }
}