use pipeline::types::*;

pub fn store_metadata(
    _input_tick: &InputTick,
    _context: &Context,
    _labels: &Labels,
    _stats: &Stats,
    _store_lap_result: &StoreLapResult,
) -> StoreMetadataResult {
    StoreMetadataResult { is_stored: false }
}
