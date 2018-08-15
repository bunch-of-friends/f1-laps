use pipeline::types::*;

pub fn build_stats(
    input_tick: &InputTick,
    context: &Context,
    _: &PacketLabels,
) -> PacketStats {
    PacketStats {
        previous_lap: None,
        previous_sector: None
    }
}
