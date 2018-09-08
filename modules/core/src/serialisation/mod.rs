pub mod f1_2018;

use pipeline::input::Tick;

pub trait ReceivePacket: Send + Sync + Clone {
    fn new() -> Self;
    fn converto_to_tick(&mut self, datagram: &[u8], size: usize) -> Option<Tick>;
}

pub fn get_serialiser() -> impl ReceivePacket {
    // for now hardcoded to F1 2018
    f1_2018::Serialiser::new()
}

pub fn get_buffer_size() -> usize {
    // for now hardcoded to F1 2018
    f1_2018::get_buffer_size()
}
