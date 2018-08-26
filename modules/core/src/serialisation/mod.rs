pub mod f1_2018;

use pipeline::types::Tick;

pub trait ReceivePacket: Send + Sync {
    fn new() -> Self;
    fn get_buffer_size(&self) -> usize;
    fn converto_to_tick(&self, datagram: &[u8], size: usize) -> Option<Tick>;
}

pub fn get_serialiser() -> impl ReceivePacket {
    // for now hardcoded to F1 2018
    f1_2018::Serialiser::new()
}
