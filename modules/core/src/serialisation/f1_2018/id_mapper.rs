use serialisation::MapId;

pub struct IdMapper {}

impl IdMapper {
    pub fn new() -> IdMapper {
        IdMapper {}
    }
}

impl MapId for IdMapper {}
