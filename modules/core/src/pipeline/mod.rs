use aggregation::tick::Tick;
use udp::packet::Packet;

trait DataReceiver {
    fn start_receiving();
    fn receive(packet: Packet);
}

// state-less
trait PipeLine {
    fn process(entry: PipelineEntry) -> PipeLineOutput;
}

struct PipelineEntry<'a> {
    packet: &'a Packet,
    context: &'a Context,
}

struct PipeLineOutput {
    tick: Tick,
}

struct Context {
    session_context: SessionContext,
    history_context: HistoryContext,
}

// in-memory
struct SessionContext {}

// fs
struct HistoryContext {}
