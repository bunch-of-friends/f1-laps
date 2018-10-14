pub mod f1_2018;

use context::AppContext;
use pipeline::input::Tick;

pub trait ReceivePacket: Send + Sync + Clone {
    fn new() -> Self;
    fn converto_to_tick(&mut self, context: &'static AppContext, datagram: &[u8], size: usize) -> Option<Tick>;
}

pub trait MapId {}

// for now everything hardcoded to F1 2018
pub fn get_serialiser() -> impl ReceivePacket {
    f1_2018::get_serialiser()
}

pub fn get_buffer_size() -> usize {
    f1_2018::get_buffer_size()
}

pub fn get_id_mapper() -> impl MapId {
    f1_2018::get_id_mapper()
}
