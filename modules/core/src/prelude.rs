//! A convenience module that re-exports the most commonly-used APIs.

pub use aggregation::tick::{Lap, LiveData, Sector, Session};
pub use lap_metadata::LapMetadata;
pub use record_tracking::record_tracker::RecordMarker;
pub use udp::packet::Car;
