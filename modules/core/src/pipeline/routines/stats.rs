use pipeline::types::*;

pub(crate) fn build_stats(
    input_tick: &InputTick,
    context: &Context,
    _: &PacketLabels,
) -> PacketStats {
    PacketStats {
        previous_lap: None,
        previous_sector: None,
        session: Session::from_input_tick(&input_tick),
    }
}
