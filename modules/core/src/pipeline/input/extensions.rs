use pipeline::input::*;

impl Tick {
    pub fn new(
        header: Header,
        session_data: Option<SessionData>,
        lap_data: LapData,
        car_motion: CarMotion,
        car_telemetry: CarTelemetry,
        car_status: Option<CarStatus>,
    ) -> Tick {
        Tick {
            header: header,
            session_data: session_data,
            lap_data: lap_data,
            car_motion: car_motion,
            car_telemetry: car_telemetry,
            car_status: car_status,
        }
    }
}
