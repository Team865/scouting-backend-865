use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Auto2025 {
    pub coral1: u32,
    pub coral2: u32,
    pub coral3: u32,
    pub coral4: u32,
    pub barge: u32,
    pub mobility: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Teleop2025 {
    pub coral1: u32,
    pub coral2: u32,
    pub coral3: u32,
    pub coral4: u32,
    pub barge: u32,
    pub algae_processor: u32,
    pub fouls: u32,
    pub tech_fouls: u32,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "game")]
pub enum GameSpecificData {
    #[default]
    None,
    Reefscape2025(GameData2025),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum AlliancePosition {
    #[default]
    None,
    Red1,
    Red2,
    Red3,
    Blue1,
    Blue2,
    Blue3,
}

impl Into<String> for AlliancePosition {
    fn into(self) -> String {
        match self {
            AlliancePosition::None => String::from("None"),
            AlliancePosition::Red1 => String::from("Red1"),
            AlliancePosition::Red2 => String::from("Red2"),
            AlliancePosition::Red3 => String::from("Red3"),
            AlliancePosition::Blue1 => String::from("Blue1"),
            AlliancePosition::Blue2 => String::from("Blue2"),
            AlliancePosition::Blue3 => String::from("Blue3"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(into = "Vec<String>")]
pub struct GameData {
    pub scouter: String,
    pub team: String,
    pub match_number: String,
    pub alliance_position: AlliancePosition,
    pub game_data: GameSpecificData,
}

impl Into<Vec<String>> for GameData {
    fn into(self) -> Vec<String> {
        let mut fields = vec![
            self.scouter,
            self.team,
            self.match_number,
            self.alliance_position.into(),
        ];
        match self.game_data {
            GameSpecificData::None => {}
            GameSpecificData::Reefscape2025(data) => fields.append(&mut vec![
                data.auto.coral1.to_string(),
                data.auto.coral2.to_string(),
                data.auto.coral3.to_string(),
                data.auto.coral4.to_string(),
                data.auto.barge.to_string(),
                data.auto.mobility.to_string(),
                data.teleop.coral1.to_string(),
                data.teleop.coral2.to_string(),
                data.teleop.coral3.to_string(),
                data.teleop.coral4.to_string(),
                data.teleop.barge.to_string(),
                data.teleop.algae_processor.to_string(),
                data.teleop.fouls.to_string(),
                data.teleop.tech_fouls.to_string(),
                data.teleop.defense.to_string(),
                data.endgame.park.to_string(),
                data.endgame.shallow.to_string(),
                data.endgame.deep.to_string(),
                data.endgame.time_climbed_at.to_string(),
            ]),
        };

        fields
    }
}
