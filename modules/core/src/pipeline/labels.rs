use pipeline::types::*;

pub(crate) fn build_labels(input_tick: &InputTick, context: &Context) -> PacketLabels {
    PacketLabels {
        is_new_session: false,
        is_new_lap: false,
        is_new_sector: false,
    }
}
