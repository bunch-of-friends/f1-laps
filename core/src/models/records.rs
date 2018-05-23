// best times for given track/era, each item in the array is one session type
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct RecordSet {
    pub records: [Record; 5], // overall, unknown (TT), practise, qualification, race
}

// best times for given track/era & session type, each array item is compound time
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Record {
    pub best_lap_time: [LapRecord; 8], //overall best lap time and the lap sector times, compounds 0 .. 6

    pub best_sector1_time: [f32; 8], //best even sector times, not necessarily from the best or even same lap
    pub best_sector2_time: [f32; 8],
    pub best_sector3_time: [f32; 8],
}

// lap records consists of best overall time and sector times from that lap
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct LapRecord {
    pub lap_time: f32,
    pub sector1_time: f32,
    pub sector2_time: f32,
    pub sector3_time: f32,
}
