use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Packet {
    #[serde(with = "ts_nanoseconds")]
    pub timestamp: DateTime<Utc>,
    pub bytes: Vec<u8>,
}

impl Packet {
    pub fn new(bytes: Vec<u8>) -> Packet {
        Packet {
            timestamp: Utc::now(),
            bytes: bytes,
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}
