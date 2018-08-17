use pipeline::types::*;

pub fn build_stats(
    input_tick: &InputTick,
    context: &Context,
    _: &PacketLabels,
) -> PacketStats {
    PacketStats {
        finished_lap: None,
        finished_sector: None
    }
}
