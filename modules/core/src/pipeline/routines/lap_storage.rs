use pipeline::types::*;

pub fn store_lap(
    _input_tick: &InputTick,
    _context: &Context,
    _labels: &Labels,
    _stats: &Stats,
) -> StoreLapResult {
    StoreLapResult { is_stored: false }
}
