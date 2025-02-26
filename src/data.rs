use non_empty_string::NonEmptyString;
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
    #[serde(rename = "Red 1")]
    Red1,
    #[serde(rename = "Red 2")]
    Red2,
    #[serde(rename = "Red 3")]
    Red3,
    #[serde(rename = "Blue 1")]
    Blue1,
    #[serde(rename = "Blue 2")]
    Blue2,
    #[serde(rename = "Blue 3")]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(into = "Vec<String>")]
pub struct GameData {
    pub scouter: NonEmptyString,
    pub team: NonEmptyString,
    pub match_number: NonEmptyString,
    pub alliance_position: AlliancePosition,
    pub commentary: String,
    pub is_test: bool,
    pub game_data: GameSpecificData,
}

impl Into<Vec<String>> for GameData {
    fn into(self) -> Vec<String> {
        let mut fields = vec![
            self.scouter.into(),
            self.team.into(),
            self.match_number.into(),
            self.alliance_position.into(),
            self.commentary,
        ];
        match self.game_data {
            GameSpecificData::None => {}
            GameSpecificData::Reefscape2025(data) => fields.append(&mut data.into()),
        };

        fields
    }
}

impl Into<Vec<String>> for GameData2025 {
    fn into(self) -> Vec<String> {
        vec![
            self.auto.coral1.to_string(),
            self.auto.coral2.to_string(),
            self.auto.coral3.to_string(),
            self.auto.coral4.to_string(),
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
