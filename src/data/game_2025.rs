use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Auto2025 {
    pub coral1: u32,
    pub coral2: u32,
    pub coral3: u32,
    pub coral4: u32,
    pub barge: u32,
    pub processor: u32,
    pub mobility: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Teleop2025 {
    pub coral1: u32,
    pub coral2: u32,
    pub coral3: u32,
    pub coral4: u32,
    pub barge: u32,
    pub processor: u32,
    pub fouls: u32,
    pub defense: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Endgame2025 {
    pub park: bool,
    pub shallow: bool,
    pub deep: bool,
    pub time_climbed_at: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameData2025 {
    pub auto: Auto2025,
    pub teleop: Teleop2025,
    pub endgame: Endgame2025,
}

impl Into<Vec<String>> for GameData2025 {
    fn into(self) -> Vec<String> {
        vec![
            self.auto.coral1.to_string(),
            self.auto.coral2.to_string(),
            self.auto.coral3.to_string(),
            self.auto.coral4.to_string(),
            self.auto.processor.to_string(),
            self.auto.barge.to_string(),
            self.auto.mobility.to_string(),
            self.teleop.coral1.to_string(),
            self.teleop.coral2.to_string(),
            self.teleop.coral3.to_string(),
            self.teleop.coral4.to_string(),
            self.teleop.barge.to_string(),
            self.teleop.processor.to_string(),
            self.teleop.fouls.to_string(),
            self.teleop.defense.to_string(),
            self.endgame.park.to_string(),
            self.endgame.shallow.to_string(),
            self.endgame.deep.to_string(),
            self.endgame.time_climbed_at.to_string(),
        ]
    }
}
