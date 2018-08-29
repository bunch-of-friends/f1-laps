use pipeline::input::*;

impl Tick {
    pub fn new(header: Header) -> Tick {
        Tick {
            header: header,
            session_data: None,
            lap_data: None,
            car_motion: None,
            car_telemetry: None,
            car_status: None,
        }
    }
}

